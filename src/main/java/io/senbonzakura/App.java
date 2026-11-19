package io.senbonzakura;

/** The program's entry point: runs the {@link Cli} on the arguments and exits with its code. */
public final class App {

  private App() {}

  public static void main(String[] args) {
    System.exit(new Cli(System.in, System.out, System.err).run(args));
  }
}
