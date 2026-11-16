package io.senbonzakura.ast;

final class NodeRange {

  private NodeRange() {}

  static void check(int start, int end) {
    if (start < 0 || end < start) {
      throw new IllegalArgumentException("invalid node range " + start + ".." + end);
    }
  }
}
