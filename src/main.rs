use std::error::Error;
use swayipc::Connection;

type R<T> = Result<T, Box<dyn Error>>;

fn main() -> R<()> {
    let mut connection = Connection::new()?;
    let root = connection.get_tree()?;
    let outputs = root
        .nodes
        .iter()
        .filter(|node| node.name != Some("__i3".to_owned()))
        .map(|node| node.name.as_ref().ok_or("output without name"))
        .collect::<Result<Vec<&String>, &str>>()?;
    match *outputs {
        [a, b] => switch_outputs(&mut connection, a, b)?,
        [a] => eprintln!("only one output: {:?}", a),
        _ => eprintln!("unexpected number of outputs: {:?}", outputs.iter(),),
    }
    Ok(())
}

fn switch_outputs(connection: &mut Connection, a: &str, b: &str) -> R<()> {
    let current_workspace = connection
        .get_workspaces()?
        .into_iter()
        .find(|workspace| workspace.focused)
        .ok_or("no focused workspace found")?;
    for workspace in connection.get_workspaces()? {
        let other_output = if workspace.output == a { b } else { a };
        connection.run_command(format!("workspace {}", workspace.name))?;
        connection.run_command(format!("move workspace to output {}", other_output))?;
    }
    connection.run_command(format!("workspace {}", current_workspace.name))?;
    Ok(())
}
