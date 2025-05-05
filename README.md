# timu
This is the language for what I would like to work on in the future. It will have many missing features but will add them when needed. My goal is to design a programming language that can solve many problems at compile time. 

### General design
At sign (@) used for compiler macros. There are many inbuild macros defined and user can create it is own macro.
Question mark (?) used for nulltypes.
<type> used for type interferance. It is available on compile time.

### Nullable type
The nullable types starts with question mark (?). The variable may have a value or empty. The compiler gives guarantee about variable has value or not.
@is_some macro check for value and return true if has it, otherwise return false
@is_none macro check for value and return false if has it, otherwise return true

### Macros
## #value and #empty
It is used for nullable type. It checks for is value available or not.
Example:

```timu
func main () {
    var a: ?int = Empty;
    var a: ?int;
    var b: ?int = 1024;
    var c: ?int = Value(1024);
    var d: bool = HasValue(1024);

    if Value(1024) == c {

    } else {

    }
}
```


### Type
The types are 

```timu
class MyType {
    a: i32 = 100, # with default value
    pub b: string, # public accessible
    c: ?string, # nullable string

    func init(): MyType { # constructor
        return MyType {
            a: 100,
            b: "erhan",
            c: None
        }
    }

    func hello(this): MyType { # constructor
        this.a = 100;
        return MyType {
            a: 100,
            b: "erhan",
            c: None
        }
    }
}

# Type interface
interface IData: Cloneable {
    a: i32 = 100;
    func hello();
    func world();
}

# Implement the interface
extend MyType: IData {
    func hello() {
        return "hello"
    }

    func world() {
        return "world"
    }
}

func func_a(a: i32): i32 {
    return a * 2
}

func func_b(a: i32): i32 {
    return a * 3
}

func main() {
    var d = MyType {
        a: 100,
        b: "erhan",
        c: None
    }

    var v = MyType()

    d.print()
    MyType.hello()
    MyType.world()

    var a = func_a(2) => func_b() ## a will be 12

    match d.b {
        "a": {
            return 100
        }
        "b": {
            return 200
        }
        _ => {
            return 300
        }
    }
}

```

### Module import


### Example code

use std  as s;
use std ;
use std.array;
use std.integer;
use std.*;

@mut static ada = false;

static ada = 1024.1;
@mut static DATA = "erhan";
static DATA = "";
static DATA = "
";

static my_list = [1,2,3,4,5];
static my_list_2: [i32; 2] = [1,2];

@clone
@compare
type data {
    a: i32 = 100,
    b: string,
    c: ?string,

    func init(): data {
        !data
    }
};

@on_change(data, (item: i32) {
    
});

@on_drop(data, (item: i32) {
    
})


// Import library

@use(*); // Supported but not recommended
@use(std);
@use(std.math);
@use(std.*);

@use(std.math) as math; // Alias for library

// Static variable. Al statis variables are immutable
STRING_DATA = ""; // Type can be detected via compiler
INTEGER_DATA: i32 = 1024; // Type is assigned as i32


// Make atomic static variable. So, developer can change variable if need it
@atomic BOOL_DATA: bool = false;


// Function definition (private)
func calculate(data: int32): int32 {
	// return type is int32
}

// Function definition (public)
pub func calculate(data: int32): int32 {
	// return type is int32
}


func datas() {
	var string_data = "";
	var multiplinestring_data = "
";
	var bool_data = true;
	var int_data = 1024;
	var float_data = 1024.0;
	var array_value = [1,2,3,4,5];
	
	var nullable_data: ?bool = None;
	
	var nullable_data = @get(nullable_data) or 10;
	var has_value = @is_empty(nullable_data);
    @defer {
        std.console.write("out of scope");
    }
}

// Integer types
@clone type i256;
@clone type u256;
@clone type i128;
@clone type u128;
@clone type i64;
@clone type u64;
@clone type i32;
@clone type u32;
@clone type i16;
@clone type u16;
@clone type i8;
@clone type u8;
@clone type char;
@clone type bool;


type Data {
	var data: i32
};


@clone // all objects will be default non-clone. Only primitive types are cloneable.
@atomic // set variable to atomic type
@get // get information from variable
@is_empty // check the variable to none
@on_change // when the variable changed invoke the function
@on_delete // when the variable deleted invoke the function
@defer // when out of scope execute the code
@error_defer // when throw the exception

@new // allocate memory
@new_ptr // allocate pointer
@free // free up the memory

var data = @new(u32); // All types must have default parameters
var data = @new_ptr(u32; 1);


// Checked pointer and unchecked pointer
All pointers news to be checked agains to null.

@new_ptr will return nullable pointer


var new_pointer = @new_ptr(str);
is equal to 

var new_pointer: !*string = @new_ptr(string); //unchecked pointer
var new_pointer: *string  = @get_ptr(new_pointer); // checked pointer and if the pointer is none, it will throw exception

secure_ptr
unsecure_ptr


// Error result
func return_error() -> Result(Nil, i32) {
    return Err(1024)
}

func handle_error() {
    var val = try return_error(1024) catch (val: i32) {
        return 1024;
    };
}

func custom_thread() {

}

func spawn_thread() {
    var my_thread = thread custom_thread()
    var my_coroutine = go custom_thread()
}



// Null result


### Code coverage
grcov .  --binary-path ./target/debug/deps/ --source-dir . --excl-start 'mod test* \{' --ignore '*test*'  --ignore "*test.rs" --ignore "*main.rs" --ignore "*tests.rs" --ignore "*github.com*" --ignore "*libcore*" --ignore "*rustc*" --ignore "*liballoc*" --ignore "*cargo*" -t html  -o ./coverage