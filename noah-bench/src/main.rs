// Importaciones
use noah_protocol::grpc::noah_service_client::NoahServiceClient;
use noah_protocol::grpc::{GetRequest, SetRequest};
use clap::Parser;
use colored::*;
use rand::{rngs::SmallRng, SeedableRng, Rng};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use hdrhistogram::Histogram;

// Argumentos
#[derive(Parser, Debug)]
#[command(name = "NoahBench")]
#[command(about = " Herramienta de Stress Testing para NoahDb")]
struct Args {
    /// Direccion del servidor (ej: http://127.0.0.1:50051)
    #[arg(short, long, default_value = "http://127.0.0.1:50051")]
    target: String,

    /// Numeros de operaciones por hilo
    #[arg(short, long, default_value_t = 1000)]
    requests: usize,

    /// Nivel de concurrencia (hilos simultaneos)
    #[arg(short, long, default_value_t = 100)]
    concurrency: usize,

    /// Tipo de operacion (set, get, mixed)
    #[arg(short, long, default_value = "mixed")]
    operation: String,
}
// Funciones
async fn benchmark_set(client: &mut NoahServiceClient<tonic::transport::Channel>, key: String, value: String) -> Duration {
    let start = Instant::now();
    let request = tonic::Request::new(SetRequest { key, value });
    let _ = client.set(request).await;
    start.elapsed()
}
// Funciones
async fn benchmark_get(client: &mut NoahServiceClient<tonic::transport::Channel>, key: String) -> Duration {
    let start = Instant::now();
    let request = tonic::Request::new(GetRequest { key });
    let _ = client.get(request).await;
    start.elapsed()
}
// Funcion de hilo
async  fn run_worker (
    target: String,
    requests: usize,
    operation: String,
    semaphore: Arc<Semaphore>,
) -> Vec<Duration> {
    // Obtener permit
    let _permit = semaphore.acquire().await.unwrap();
    // Crear client
    let mut client = NoahServiceClient::connect(target).await.unwrap();
    // Crear latencias
    let mut latencies = Vec::with_capacity(requests);
    // Crear rng
    let mut rng = SmallRng::from_entropy();
    // Ejecutar operaciones
    for i in 0..requests {
        // Obtener latencia
        let latency = match operation.as_str() {
            // Generar key
            "set" => {
                // Generar key
                let key = format!("key_{}", rng.gen::<u32>());
                // Generar value
                let value = format!("value_{}", i);
                benchmark_set(&mut client, key, value).await
            }
            // Generar key
            "get" => {
                let key = format!("key_{}", rng.gen::<u32>() % 10000);
                benchmark_get(&mut client, key).await
            }
            // Generar key
            "mixed" => {
                // Generar key
                if rng.gen_bool(0.7) {
                    let key = format!("key_{}", rng.gen::<u32>());
                    let value = format!("value_{}", i);
                    benchmark_set(&mut client, key, value).await
                } else {
                    // Generar key
                    let key = format!("key_{}", rng.gen::<u32>() % 10000);
                    benchmark_get(&mut client, key).await
                }
            }
            // Generar key
            _=> Duration::from_millis(0),
        };
        // Agregar latencia
        latencies.push(latency);
    }
    // Devolver latencias
    latencies
}
// Funcion principal
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parsear argumentos
    let args = Args::parse();
    // Imprimir encabezado
    println!("{}", " NoahDB Benchmark Tool".bright_red().bold());
    println!(" Target: {}", args.target.bright_blue());
    println!(" Concurrency: {}", args.concurrency.to_string().bright_green());
    println!(" Requests: per worker: {}", args.requests.to_string().bright_yellow());
    println!(" Operation: {}", args.operation.bright_magenta());
    println!("{}", "=".repeat(50).bright_black());
    // Crear semaforo
    let semaphore = Arc::new(Semaphore::new(args.concurrency));
    let start_time = Instant::now();
    // Crear hilos
    let mut tasks = Vec::new();
    let requests = args.requests;
    for _ in 0..args.concurrency {
        // Clonar argumentos
        let target = args.target.clone();
        let operation = args.operation.clone();
        let semaphore = semaphore.clone();
        // Crear un hilo
        let task = tokio::spawn(async move {
            run_worker(target, requests, operation, semaphore).await
        });
        tasks.push(task);
    }
    // Esperar a que todos los hilos terminen
    let mut all_latencies = Vec::new();
    for task in tasks {
        let latencies = task.await?;
        all_latencies.extend(latencies);
    }
    // Calcular tiempos
    let total_time = start_time.elapsed();
    let total_requests = all_latencies.len();

    // Calcular estadisticas
    let mut histogram = Histogram::<u64>::new(3).unwrap();
    for latency in &all_latencies {
        histogram.record(latency.as_micros() as u64).unwrap();
    }
    // Calcular throughput
    let throughput = total_requests as f64 / total_time.as_secs_f64();
    // Imprimir resultados
    println!("{}", " RESULTADOS".bright_green().bold());
    println!("  Tiempo total: {:.2?}", total_time);
    println!("  Total requests: {}", total_requests.to_string().bright_cyan());
    println!("  Throughput: {} req/s", format!("{:.0}", throughput).bright_green());
    println!("  Latencia promedio: {:.2} ms",histogram.mean() / 1000.0);
    println!("  P50: {:.2} ms", histogram.value_at_quantile(0.5) as f64 / 1000.0);
    println!("  P95: {:.2} ms", histogram.value_at_quantile(0.95) as f64 / 1000.0);
    println!("  P99: {:.2} ms", histogram.value_at_quantile(0.99) as f64 / 1000.0);
    // Imprimir histograma
    Ok(())
}