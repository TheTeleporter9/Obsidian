Initialize an empty list called 'tokens_output'
Initialize 'current_position' at index 0

While 'current_position' is less than the total length of the source text:
    Get the character at 'current_position'

    RULE 1: Skip Whitespace
    If the character is a space, tab, or newline:
        Move 'current_position' forward by 1
        Continue to the next iteration of the loop

    RULE 2: Match Numbers
    If the character is a digit (0-9):
        Keep reading characters forward until you hit a non-digit character
        Extract that full block of digits (e.g., "125")
        Create a new Token object:
            Type: LITERAL_INT
            Value: "125"
        Add this Token to 'tokens_output'
        Update 'current_position' to the end of the digits

    RULE 3: Match Words (Identifiers & Keywords)
    If the character is a letter (a-z, A-Z):
        Keep reading characters forward as long as they are letters or numbers
        Extract that full word block (e.g., "const" or "my_variable")
        
        Check if the word matches a language keyword:
            If word is "const", Type is KEYWORD_CONST
            If word is "var", Type is KEYWORD_VAR
            Otherwise, Type is IDENTIFIER
            
        Create a new Token object with the Type and the word string as Value
        Add this Token to 'tokens_output'
        Update 'current_position' to the end of the word

    RULE 4: Match Operators & Symbols
    If the character starts a symbol sequence:
        Check if the current character and the NEXT character make "->"
            If yes: 
                Create Token(Type: OPERATOR_PIPELINE, Value: "->")
                Move 'current_position' forward by 2
                Continue
        Check if the character is "="
            If yes:
                Create Token(Type: OPERATOR_ASSIGN, Value: "=")
                Move 'current_position' forward by 1
                Continue

    RULE 5: Error Handling
    If none of the rules matched the character (e.g., an unexpected symbol like "@" or "$"):
        Stop the program and raise a "Lexer Error: Unexpected character"

Return 'tokens_output'

// A Node can be one of these specific types
Structure Node:
    Type: AssignmentNode OR VariableDeclarationNode OR PrintNode OR ExpressionNode

// If it's a Variable Declaration (e.g., VAR main = 4)
Structure VariableDeclarationNode:
    VariableName: String
    Value: Node // This points to another node, like a number or a math equation

// If it's a Print statement (e.g., PRINT main)
Structure PrintNode:
    Target: Node // The thing we want to print

// If it's an Operations/Math statement (e.g., main + 5)
Structure BinaryOperationNode:
    Left: Node      // e.g., the variable 'main'
    Operator: String // e.g., "+"
    Right: Node     // e.g., the literal '5'

// If it's just a raw number or variable name
Structure LiteralIntNode:
    Value: Integer

Structure IdentifierNode:
    Name: String


Function Parse(tokens):
    Position = 0
    AST_Root_Nodes = empty list
    
    While Position < length of tokens:
        // Look at the current token without moving forward
        CurrentToken = tokens[Position]
        
        If CurrentToken is "VAR":
            // We found a variable declaration! Call its special function
            Node = ParseVariableDeclaration()
            AST_Root_Nodes.append(Node)
            
        Else If CurrentToken is "PRINT":
            // We found a print statement! Call its special function
            Node = ParsePrintStatement()
            AST_Root_Nodes.append(Node)
            
        Else:
            Error("Hey! I don't know what this token is doing here!")
            
    Return AST_Root_Nodes


Function ParseVariableDeclaration():
    Match("VAR") // Consume the "VAR" token and move forward
    
    VarNameToken = tokens[Position]
    Match("Identifier") // Consume the variable name (like "main")
    
    Match("OperatorAssign") // Consume the "=" sign
    
    // Now we parse whatever comes after the "=" sign!
    ValueNode = ParseExpression() 
    
    // Create our beautiful tree node piece!
    Return New VariableDeclarationNode(VariableName: VarNameToken.value, Value: ValueNode)


Function ParsePrintStatement():
    Match("PRINT") // Consume "PRINT"
    
    TargetNode = ParseExpression() // Parse what needs to be printed
    
    Return New PrintNode(Target: TargetNode)


Function ParseExpression():
    // For now, let's just parse simple numbers or variables
    CurrentToken = tokens[Position]
    
    If CurrentToken is "LiteralInt":
        Match("LiteralInt")
        LeftNode = New LiteralIntNode(Value: CurrentToken.value)
    Else If CurrentToken is "Identifier":
        Match("Identifier")
        LeftNode = New IdentifierNode(Name: CurrentToken.value)
        
    // Look ahead: Is the next token a math operator like "+"?
    If tokens[Position] is "OperatorAdd":
        Match("OperatorAdd")
        RightNode = ParseExpression() // Recursively get the right side!
        
        // Wrap them together in a math node
        Return New BinaryOperationNode(Left: LeftNode, Operator: "+", Right: RightNode)
        
    Return LeftNode


Enum ASTNode:
    // Represents: VAR name = value
    VariableDeclaration(name: String, value: ASTNode) 
    
    // Represents: PRINT value
    Print(target: ASTNode)
    
    // Represents: left + right
    BinaryOperation(left: ASTNode, operator: String, right: ASTNode)
    
    // Represents: main
    Identifier(name: String)
    
    // Represents: 4
    LiteralInt(value: Integer)

Function Parse(tokens):
    position = 0
    program_nodes = empty list
    
    While position < length(tokens):
        CurrentToken = tokens[position]
        
        // Match against the type of the current token
        Match CurrentToken:
            If "VAR":
                Node = ParseVariableDeclaration()
                program_nodes.append(Node)
                
            If "PRINT":
                Node = ParsePrintStatement()
                program_nodes.append(Node)
                
            If Anything Else:
                Error("Unexpected token!")
                
    Return program_nodes


Function ParseVariableDeclaration():
    advance() // Skip past the "VAR" token
    
    // The current token MUST be an Identifier now
    CurrentToken = tokens[position]
    If CurrentToken is NOT "Identifier":
        Error("Expected a variable name!")
    VarName = CurrentToken.value
    advance() // Skip past the Identifier
    
    // The current token MUST be an "=" sign
    If tokens[position] is NOT "OperatorAssign":
        Error("Expected an '=' sign!")
    advance() // Skip past the "="
    
    // Go find the value expression (like "4" or "main + 5")
    ValueNode = ParseExpression()
    
    Return ASTNode.VariableDeclaration(name: VarName, value: ValueNode)


Function ParsePrintStatement():
    advance() // Skip past the "PRINT" token
    
    // Go find what we are printing
    TargetNode = ParseExpression()
    
    Return ASTNode.Print(target: TargetNode)


Function ParseExpression():
    // Step A: Get the left-side item (a number or a variable name)
    CurrentToken = tokens[position]
    LeftNode = null
    
    Match CurrentToken:
        If "LiteralInt":
            LeftNode = ASTNode.LiteralInt(value: CurrentToken.value)
            advance()
        If "Identifier":
            LeftNode = ASTNode.Identifier(name: CurrentToken.value)
            advance()
        If Anything Else:
            Error("Expected a number or a variable!")

    // Step B: Look ahead! Is there a math operator next?
    If position < length(tokens) AND tokens[position] is "OperatorAdd":
        advance() // Skip past the "+" token
        
        // Recursively get whatever is on the right side of the "+"
        RightNode = ParseExpression()
        
        // Wrap the left and right sides together!
        Return ASTNode.BinaryOperation(left: LeftNode, operator: "+", right: RightNode)
        
    // If there was no "+", just return the single number or variable
    Return LeftNode


Function TranspileToC(ast_nodes):
    // 1. Add the standard library header that C needs for printf
    C_Code_Output = "#include <stdio.h>\n\n"
    
    // 2. Open the main function block for C
    C_Code_Output.append("int main() {\n")
    
    // 3. Loop through every single node in your AST list
    For Each Node in ast_nodes:
        // Convert the node into a line of C code
        C_Line = ConvertNodeToCString(Node)
        
        // Append it with 4 spaces of indentation and a newline
        C_Code_Output.append("    " + C_Line + "\n")
        
    // 4. Close the C main function block cleanly
    C_Code_Output.append("    return 0;\n")
    C_Code_Output.append("}\n")
    
    Return C_Code_Output


Function ConvertNodeToCString(Node):
    Match Node Type:
    
        If VariableDecleration(name, value_node):
            // Recursively turn the value inside (like LiteralInt) into a string
            ValueText = ConvertNodeToCString(value_node)
            // C variables need a type (like int) and a semicolon at the end!
            Return "int " + name + " = " + ValueText + ";"
            
        If PrintDecleration(target_node):
            TargetText = ConvertNodeToCString(target_node)
            // Turn your print node into a standard C printf statement
            Return "printf(\"%d\\n\", " + TargetText + ");"
            
        If Identifier(name):
            Return name
            
        If LiteralInt(value):
            // Convert the raw integer number into text strings
            Return ToString(value)
            
        If Anything Else:
            Error("Uh oh! I don't know how to turn this AST node into C yet!")