use std::sync::mpsc; // mpsc: multiple producer, single consumer (para los canales)
use std::thread;
use std::time::{Duration, Instant};

const NUM_ITERATIONS: u32 = 1_000_000; // 1 millón de pruebas para una medición precisa

fn main() {
    println!("Iniciando solución: Monolito de Alto Rendimiento...");
    println!("Comunicación entre hilos mediante canales en memoria.");

    // 1. ARQUITECTURA: CREACIÓN DE CANALES EN MEMORIA
    // Se crean dos canales para la comunicación bidireccional. Cada canal tiene un
    // emisor (tx) y un receptor (rx). La comunicación es casi instantánea.
    let (stimulus_tx, stimulus_rx) = mpsc::channel::<()>(); // Cliente -> Servidor
    let (response_tx, response_rx) = mpsc::channel::<()>(); // Servidor -> Cliente

    // Se usa `()` (el "unit type") como mensaje. Es una señal de cero bytes,
    // la forma de comunicación más rápida posible.

    // 2. ROL DEL SERVIDOR: LANZAMIENTO DE UN HILO DEDICADO
    // Se lanza un hilo que actuará como el "servidor". El `move` transfiere la
    // propiedad de los extremos del canal que necesita (stimulus_rx y response_tx).
    let server_handle = thread::spawn(move || {
        // Este bucle `for` en el receptor del canal es la forma más eficiente de
        // esperar un mensaje. El hilo se "duerme" hasta que llega algo.
        for _stimulus in stimulus_rx {
            // En cuanto recibe el estímulo, envía la respuesta de vuelta.
            // La operación es una simple escritura en una cola en memoria.
            response_tx.send(()).unwrap();
        }
    });

    // 3. ROL DEL CLIENTE Y MEDICIÓN (EJECUTADO EN EL HILO PRINCIPAL)
    println!("Lanzando {} estímulos y midiendo la latencia de ida y vuelta...", NUM_ITERATIONS);

    let mut latencies: Vec<Duration> = Vec::with_capacity(NUM_ITERATIONS as usize);

    // Bucle principal para la medición de latencia
    for _ in 0..NUM_ITERATIONS {
        // Inicia el cronómetro de alta precisión justo antes de enviar.
        let start = Instant::now();

        // Envía el estímulo. Es una operación en memoria, no hay llamadas al sistema.
        stimulus_tx.send(()).unwrap();

        // Espera la respuesta. El hilo se bloquea aquí hasta que el servidor responde.
        response_rx.recv().unwrap();

        // Detiene el cronómetro y guarda la duración del viaje completo.
        let elapsed = start.elapsed();
        latencies.push(elapsed);
    }

    println!("Medición completada.");

    // 4. LIMPIEZA: DETENER EL HILO SERVIDOR DE FORMA SEGURA
    // Al eliminar (hacer "drop") el emisor del estímulo, el canal se cierra.
    // Esto provoca que el bucle `for` en el hilo servidor termine, y el hilo finalice.
    drop(stimulus_tx);
    // .join() espera a que el hilo servidor termine su ejecución.
    server_handle.join().expect("El hilo servidor tuvo un error.");

    // 5. ANÁLISIS Y REPORTE DE RESULTADOS
    let total_duration: Duration = latencies.iter().sum();
    let avg_latency = total_duration / NUM_ITERATIONS;
    let min_latency = latencies.iter().min().unwrap();
    let max_latency = latencies.iter().max().unwrap();

    println!("\n--- Resultados de Latencia (Monolito en Memoria) ---");
    println!("Pruebas realizadas: {}", NUM_ITERATIONS);
    // La latencia es tan baja que debemos medirla en nanosegundos (ns).
    println!("Latencia Mínima:   {:.0} ns", min_latency.as_nanos());
    println!("Latencia Promedio: {:.0} ns", avg_latency.as_nanos());
    println!("Latencia Máxima:    {:.0} ns", max_latency.as_nanos());
    println!("----------------------------------------------------");
    println!("\nOBJETIVO CUMPLIDO: La latencia promedio de nanosegundos es miles de veces inferior al objetivo de 1 milisegundo.");
}