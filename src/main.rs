use std::process::Command;

fn main() {
    let container = std::env::args().nth(1).expect("Esperado: container-name");
    let _action = std::env::args().nth(2).expect("Esperado: action");

    if container == "ls" {
        let output = Command::new("sh")
            .args(["-c", "docker ps", "-a"]) 
            .spawn()
            .expect("Deu nao pae");

        println!("{:?}", output.stdout);
    } else {
        let output = Command::new("sh")
            .args(["-c", &format!("docker {_action} {container}")]) 
            .spawn()
            .expect("Deu nao pae");

        println!("{:?}", output.stdout);
    }
}
