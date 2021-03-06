use crate::context::*;
use crate::debug_adapter_comms;
use crate::kakoune;

use json::object;

//Initializes the debug adapter.
pub fn initialize(ctx: &mut Context) {
    //Construct the initialize request
    let initialize_args = object!{
        "adapterID": "pydbg",
        "linesStartAt1": true,
        "columnsStartAt1": true,
        "pathFormat": "path",
        "supportsRunInTerminalRequest": true,
    };
    debug_adapter_comms::do_request("initialize".to_string(), initialize_args, ctx);
}

//Handles the "initialized" event.
pub fn handle_initialized_event(_msg: json::JsonValue, ctx: &mut Context) {
    //This is where we'd set the breakpoints
    //Breakpoints hardcoded for now; TODO: receive breakpoints from editor.
    let break_args = object!{
        "source": {
            "name": "test",
            "path": "/home/jdugan/projects/kak_plugins/kak-dap/demo/python/test.py"
        },
        "breakpoints": [
            {
                "line": 10,
            }
        ]
    };
    debug_adapter_comms::do_request("setBreakpoints".to_string(), break_args, ctx);

    //Now, send the configurationDone request.
    debug_adapter_comms::do_request("configurationDone".to_string(), object!{}, ctx);
}

//Handles the "initialize" response.
pub fn handle_initialize_response(_msg: json::JsonValue, ctx: &mut Context) {
    //We need to send the launch request before the breakpoints.
    //For background: https://github.com/microsoft/vscode/issues/4902
    let launch_args = object!{
        "program": "/home/jdugan/projects/kak_plugins/kak-dap/demo/python/test.py",
        "args": [],
        "stopOnEntry": false,
        "console": "externalTerminal",
        "debugOptions": [],
        "cwd": "/home/jdugan/projects/kak_plugins/kak-dap/demo/python"
    };
    debug_adapter_comms::do_request("launch".to_string(), launch_args, ctx);
}

//Handles the "runInTerminal" request.
pub fn handle_run_in_terminal_request(msg: json::JsonValue, ctx: &mut Context) {
    //Get the sequence number of this request to send back later
    let seq = &msg["seq"];
    ctx.last_adapter_seq = seq.to_string().parse::<u64>().unwrap();
    //Extract the program we need to run
    let args = &msg["arguments"]["args"];
    let mut cmd = "dap-run-in-terminal ".to_string();
    let args_members = args.members();
    for val in args_members {
        cmd.push_str("\"");
        cmd.push_str(&val.to_string());
        cmd.push_str("\" ");
    }
    kakoune::kak_command(cmd, &ctx);
}

//Handles the "evaluate" response.
pub fn handle_evaluate_response(msg: json::JsonValue, ctx: &mut Context) {
    //Get the result and type
    let result = &msg["body"]["result"];
    let typ = &msg["body"]["type"];
    
    //Send it to Kakoune for processing
    let mut cmd = "dap-evaluate-response ".to_string();
    cmd.push_str(&result.to_string());
    cmd.push_str(" ");
    cmd.push_str(&typ.to_string());
    kakoune::kak_command(cmd, &ctx);
}
