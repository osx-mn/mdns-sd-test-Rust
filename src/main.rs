use local_ip_address::local_ip;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Iniciando el daemon...");

    //ANUNCIO DEL SERVICIO A OFRECER
    let ty_domain = "_remit_transfer._tcp.local.";
    let instance_name = "Remit";
    let host_name = "Remit.local.";

    let ip = match local_ip() {
        Ok(ip) => ip.to_string(),
        Err(e) => {
            eprintln!("Error al obtener la IP local: {}", e);
            return Err(e.into());
        }
    };

    let port = 8080;
    //properties
    let properties = [("ej_prp", "campo_de_prueba")];

    //Inicialización del daemon
    let mdns_daemon = ServiceDaemon::new()?;
    println!("Daemon iniciado!");

    //"", "", "" son metadatos opcionales (si los usaré) para emitir cosas como nombre de app, de la pc y alguna otra cosa
    let service_info = ServiceInfo::new(
        ty_domain,
        instance_name,
        host_name,
        ip,
        port,
        &properties[..],
    )?;

    mdns_daemon.register(service_info)?;
    println!("Servicio anunciado!");

    //Definición del tipo de servicio a buscar
    println!("Iniciando la búsqueda de {}", ty_domain);

    //Búsqueda del servicio
    let receiver = mdns_daemon.browse(ty_domain)?;
    println!("Búsqueda iniciada!");

    while let Ok(event) = receiver.recv() {
        match event {
            ServiceEvent::ServiceFound(ty_domain, instance_name) => {
                println!("\n\n ¡Servicio encontrado!");
                println!(" - Tipo: {}", ty_domain);
                println!(" - Instancia: {}", instance_name);
                //salir del match
                // break;
            }

            ServiceEvent::ServiceResolved(resolved) => {
                println!("\n\n - Full name: {}", resolved.get_fullname());
                println!(" - Host name: {}", resolved.get_hostname());

                for ip in resolved.get_addresses() {
                    println!(" - IP: {}", ip);
                }

                println!(" - Puerto: {}", resolved.get_port());
                println!(" - Propiedades: {:?}", resolved.get_properties());
            }

            _ => {}
        }
    }

    //Opcional: detener la búsqueda especifica, ¿Porqué opcional?
    mdns_daemon.stop_browse(ty_domain)?;

    Ok(())
}
