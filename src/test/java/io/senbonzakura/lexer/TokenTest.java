package io.senbonzakura.lexer;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import io.senbonzakura.lexer.Token.Kind;
import java.util.Arrays;
import java.util.Objects;
import java.util.Set;
import org.junit.jupiter.api.Test;

class TokenTest {

  @Test
  void eofIsZeroWidth() {
    var eof = Token.eof(5, Token.NEWLINE_BEFORE);

    assertEquals(new Token(Kind.EOF, 5, 5, Token.NEWLINE_BEFORE), eof);
    assertEquals(0, eof.length());
  }

  @Test
  void length() {
    assertEquals(3, new Token(Kind.IDENTIFIER, 2, 5, 0).length());
  }

  @Test
  void flags() {
    var token = new Token(Kind.STRING_LITERAL, 0, 4, Token.UNTERMINATED | Token.HAS_ESCAPES);

    assertTrue(token.has(Token.UNTERMINATED));
    assertTrue(token.has(Token.HAS_ESCAPES));
    assertFalse(token.has(Token.NEWLINE_BEFORE));
    assertFalse(token.has(Token.SPACE_BEFORE));
  }

  @Test
  void flagsAreDistinctBits() {
    int[] flags = {Token.NEWLINE_BEFORE, Token.SPACE_BEFORE, Token.UNTERMINATED, Token.HAS_ESCAPES};
    int seen = 0;
    for (int flag : flags) {
      assertEquals(1, Integer.bitCount(flag));
      assertEquals(0, seen & flag);
      seen |= flag;
    }
  }

  @Test
  void onlyFixedTextKindsHaveASpelling() {
    assertEquals("if", Kind.IF.spelling());
    assertNull(Kind.IDENTIFIER.spelling());
    assertNull(Kind.STRING_LITERAL.spelling());
    assertNull(Kind.EOF.spelling());
  }

  @Test
  void categories() {
    assertTrue(Kind.IF.isKeyword());
    assertFalse(Kind.IF.isOperator());
    assertTrue(Kind.PLUS.isOperator());
    assertFalse(Kind.PLUS.isKeyword());
    assertFalse(Kind.IDENTIFIER.isKeyword() || Kind.IDENTIFIER.isOperator());
    assertFalse(Kind.EOF.isKeyword() || Kind.EOF.isOperator());
  }

  @Test
  void everyKindWithASpellingHasExactlyOneCategory() {
    for (var kind : Kind.values()) {
      boolean hasSpelling = kind.spelling() != null;
      assertEquals(hasSpelling, kind.isKeyword() || kind.isOperator(), kind::name);
      assertFalse(kind.isKeyword() && kind.isOperator(), kind::name);
    }
  }

  @Test
  void keywordsAreWordsAndOperatorsAreNot() {
    for (var kind : Kind.values()) {
      if (kind.spelling() != null) {
        assertEquals(kind.isKeyword(), Character.isLetter(kind.spelling().charAt(0)), kind::name);
      }
    }
  }

  @Test
  void spellingsAreUnique() {
    var spellings =
        Arrays.stream(Kind.values()).map(Kind::spelling).filter(Objects::nonNull).toList();

    assertEquals(spellings.size(), Set.copyOf(spellings).size(), spellings::toString);
  }

  @Test
  void rejectsInvalidRanges() {
    assertThrows(IllegalArgumentException.class, () -> new Token(Kind.ERROR, -1, 0, 0));
    assertThrows(IllegalArgumentException.class, () -> new Token(Kind.ERROR, 3, 2, 0));
    assertThrows(NullPointerException.class, () -> new Token(null, 0, 0, 0));
  }
}
