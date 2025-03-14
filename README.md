# timu
Timu Programming Language


@use(std) as s;
@use(std);
@use(std.array);
@use(std.integer);
@use(std.*);

@mut static ada = false;

static ada = 1024.1;
@mut static DATA = "erhan";
static DATA = "";
static DATA = "
";

static my_list = [1,2,3,4,5];
static my_list_2: [i32; 2] = [1,2];

struct data {
    a: i32 = 100,
    b: string,
    c: ?string
};

@onchange(data, (item: i32) {
    
});

@ondrop(data, (item: i32) {
    
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
	
	var nullable_data: ?bool = none;
	
	var nullable_data = @get(nullable_data) or 10;
	var has_value = @is_empty(nullable_data);
	@defer std.console.write("out of scope")
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


@clone // all objects will be default non-clone. Only primative types are cloneable.
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
// Null result

------------------------------------------------------------

# Timu Programming Language (Improved)

# Module Imports (Consistent and Clear)
module std {
    use console;
    use math as m;  # Alias example
    use array;
    use integer;
    # ... other standard modules
}

# Static Variables (Consistent Mutability and Initialization)
static ada: float64 = 1024.1; # Type explicitly defined
@mut static mutableData: string = "erhan"; # Clear mutability
static constantData: string = ""; # Immutable constant
static multiLineConstant: string = """
This is a multi-line string.
""";

static myList: [i32] = [1, 2, 3, 4, 5];
static fixedSizeList: [i32; 2] = [1, 2];

# Struct Definition (Consistent Naming)
struct Data {
    a: i32 = 100,
    b: string,
    c: ?string, # Optional string
}

# Atomic Variables (Clear Purpose)
@atomic static atomicBool: bool = false;

# Function Definitions (Clear Scope and Naming)
# Private function
func calculateData(data: i32): i32 {
    # ... implementation
    return data * 2;
}

# Public function
pub func processData(data: i32): i32 {
    # ... implementation
    return calculateData(data) + 1;
}

func dataHandling() {
    var stringData: string = "";
    var multiLineString: string = """
    Multi-line variable.
    """;
    var boolData: bool = true;
    var intData: i32 = 1024;
    var floatData: float64 = 1024.0;
    var arrayValue: [i32] = [1, 2, 3, 4, 5];

    var nullableBool: ?bool = none;

    var valueOrTen: i32 = @get(nullableBool) or 10; # clear variable name.
    var isEmpty: bool = @is_empty(nullableBool);

    @defer std.console.write("out of scope");
}

# Type Aliases (Clarity)
type Int256 = i256;
type UInt256 = u256;
type Int128 = i128;
type UInt128 = u128;
type Int64 = i64;
type UInt64 = u64;
type Int32 = i32;
type UInt32 = u32;
type Int16 = i16;
type UInt16 = u16;
type Int8 = i8;
type UInt8 = u8;
type Char = char;
type Boolean = bool;

# Memory Management (Explicit and Safe)
var dynamicInt: *i32 = @new(i32); # Checked Pointer
var dynamicArray: !*i32 = @new_ptr(i32; 10); # Unchecked pointer.

if dynamicInt != none {
    *dynamicInt = 42; # Dereference
    @free(dynamicInt);
}

if dynamicArray != none {
    dynamicArray[0] = 100;
    @free(dynamicArray);
}

# Error Handling (Example)
func safeOperation(value: i32): Result<i32, string> {
    if value < 0 {
        return Err("Negative value");
    }
    return Ok(value * 2);
}

func exampleErrorHandling() {
    var result = safeOperation(10);
    match result {
        Ok(val) => std.console.write("Result: " + val),
        Err(err) => std.console.error("Error: " + err),
    }

    var result2 = safeOperation(-5);
        match result2{
                Ok(val) => std.console.write("Result: " + val),
                Err(err) => std.console.error("Error: " + err),
        }
}

# Clone example.
@clone struct CloneableData{
        value : i32,
}

func exampleClone(){
        var original : CloneableData = CloneableData{value:10};
        var cloned : CloneableData = @clone(original);
        cloned.value = 20;
        std.console.write("Original : " + original.value);
        std.console.write("Cloned : " + cloned.value);
}

# on_change example.
@on_change(myVariableChanged)
var myVariable : i32 = 5;

func myVariableChanged(oldValue: i32, newValue: i32){
        std.console.write("Variable changed from " + oldValue + " to " + newValue);
}

func exampleOnChange(){
        myVariable = 10;
}