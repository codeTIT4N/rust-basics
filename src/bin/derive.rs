#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}
#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee) {
    //Note: There is no & here so we are not borrowing
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };
    print_employee(me);
    print_employee(me);
}
