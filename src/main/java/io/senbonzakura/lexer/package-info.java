/**
 * Turns source text into tokens.
 *
 * <ul>
 *   <li>Tokens hold offsets only. {@link SourceFile} maps them to text, and to lines and columns.
 *   <li>Errors never throw. They are collected in {@link Lexer#diagnostics()}. A malformed literal
 *       keeps its kind, and a character that starts no token becomes a {@link Token.Kind#ERROR}
 *       token.
 *   <li>Whitespace and comments produce no tokens. The token after them records them in its {@link
 *       Token#SPACE_BEFORE} and {@link Token#NEWLINE_BEFORE} flags.
 *   <li>{@link Lexer} picks a scanner from the first character of each token. {@code Cursor} holds
 *       all mutable state, and the {@code Scan*} classes are stateless.
 * </ul>
 */
package io.senbonzakura.lexer;
