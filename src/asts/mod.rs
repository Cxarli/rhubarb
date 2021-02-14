pub mod bobo;
/**!
 * The Abstract Syntax Tree (AST) is the general structure of a program.
 * It can be seen as a big graph connecting the different parts.
 *
 * There are 2 different ASTs supported by this compiler:
 * - expr: used by the input language, only supports arithmetic
 * - bobo: used by the 8080 output language
 */
pub mod expr;
