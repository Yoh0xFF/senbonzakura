/**
 * Turns tokens into a syntax tree of {@link io.senbonzakura.ast} nodes.
 *
 * <ul>
 *   <li>Errors never throw. They are collected in {@link Parser#diagnostics()}, and the part that
 *       failed becomes an {@link io.senbonzakura.ast.Expr.Error} node. Problems the lexer already
 *       reported are not reported again.
 *   <li>{@link Parser} picks a parser from the token that starts each statement, operand and
 *       primary. {@code TokenCursor} holds all mutable state, and the {@code Parse*} classes are
 *       stateless.
 *   <li>Parsers call back into {@code Parser.statement}, {@code Parser.expression}, {@code
 *       Parser.operand} and {@code Parser.primary} for nested parts, so they never depend on one
 *       another directly. {@code ParseOperator} alone knows operator precedence.
 * </ul>
 */
package io.senbonzakura.parser;
