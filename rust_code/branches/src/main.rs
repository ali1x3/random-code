fn main() {
    // conditions in rust MUST be boolean, unlike most other C based langauges
    // if blocks can return values btw (wtf) if you put expressions
    // if blocks need to end with a semi colon when used as an experssion(this is really wierd)
    // the expression would need to return the same types within the different arms
    let x = 5;

    if x < 10{
        true;
    }
    else {
        false;
    }

    if x < 10 {
        true
    }
    else {
        false
    };
}
