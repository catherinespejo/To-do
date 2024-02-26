use postgres::{ Client, NoTls };
use postgres::Error as PostgresError;
use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };
use std::env;

#[macro_use]
extern crate serde_derive;

//Modelo: definir estructura de una tarea/task
#[derive(Serialize, Deserialize)]
struct Task {
    id: Option<i32>,
    title: String,
    completed: bool,
}

//DATABASE URL
const DB_URL: &str = env!("DATABASE_URL");

//cosntants
const OK_RESPONSE: &str =
    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

//main function
fn main() {
    //Set Database
    if let Err(_) = set_database() {
        println!("Error al conectar con la base de datos");
        dbg!(DB_URL);
        return;
    }

    //correr el servidor en el puerto 
    let listener = TcpListener::bind(format!("0.0.0.0:6001")).unwrap();
    println!("Servidor esta corriendo en puerto 6001");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error al conectar: {}", e);
            }
        }
    }
}

//Controladora de todas las funciones del CRUD
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /api/tasks") => handle_post_request(r),
                r if r.starts_with("GET /api/tasks") => handle_get_all_request(r),
                r if r.starts_with("PUT /api/tasks/") => handle_put_request(r),
                r if r.starts_with("DELETE /api/tasks/") => handle_delete_request(r),
                _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Hubo un error al leer el stream: {}", e),
    }
}

//Para request tipo POST
fn handle_post_request(request: &str) -> (String, String) {
    match (get_task_request_body(&request), Client::connect(DB_URL, NoTls)) {
        (Ok(task), Ok(mut client)) => {
            client
                .execute(
                    "INSERT INTO tasks (title, completed) VALUES ($1, $2) RETURNING *",
                    &[&task.title, &task.completed]
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "Tarea creada".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

//Request tipo GET, consigue todas las tareas/tasks
fn handle_get_all_request(_request: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut tasks = Vec::new();

            for row in client.query("SELECT * FROM tasks ORDER BY id ASC", &[]).unwrap() {
                tasks.push(Task {
                    id: row.get(0),
                    title: row.get(1),
                    completed: row.get(2),
                });
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&tasks).unwrap())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

//Para requests tipo PUT
fn handle_put_request(request: &str) -> (String, String) {
    match
        (
            get_id(&request).parse::<i32>(), // Conseguir el ID de la tarea
            get_task_request_body(&request), // Conseguir el cuerpo del request
            Client::connect(DB_URL, NoTls),
        )
    {
        (Ok(id), Ok(task), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE tasks SET title = $1, completed = $2 WHERE id = $3 RETURNING *",
                    &[&task.title, &task.completed, &id]
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "Usuario actualizado".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

//Requests tipo DELETE
fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            let filas_afectadas = client.execute("DELETE FROM tasks WHERE id = $1 RETURNING *", &[&id]).unwrap();

            //Si no se actualizo ninguna fila, la tarea no existe
            if filas_afectadas == 0 {
                return (NOT_FOUND.to_string(), "La tarea no existe.".to_string());
            }

            (OK_RESPONSE.to_string(), "Tarea borrada con exito".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

//db setup
fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS tasks (
            id SERIAL PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT FALSE
        );
    "
    )?;
    Ok(())
}

//Get id from request URL
fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

//Se utilizara para de-serializar el cuerpo del request en los endpoints de Create y Update
fn get_task_request_body(request: &str) -> Result<Task, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
