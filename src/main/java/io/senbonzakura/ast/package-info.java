/**
 * The syntax tree the parser builds: {@link Stmt} nodes holding {@link Expr} and {@link Type}
 * nodes.
 *
 * <ul>
 *   <li>Nodes are immutable records grouped under sealed interfaces, so a {@code switch} over them
 *       is checked for exhaustiveness.
 *   <li>Nodes hold source offsets, not positions. {@link io.senbonzakura.lexer.SourceFile} maps
 *       them to lines and columns.
 *   <li>A part that failed to parse becomes an {@link Expr.Error} or {@link Type.Error} node rather
 *       than being dropped, so the tree keeps its shape after an error.
 * </ul>
 */
package io.senbonzakura.ast;
