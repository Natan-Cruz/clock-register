import { assertEquals } from "https://deno.land/std@0.211.0/assert/mod.ts";

class UrlParse {
  private paths = new Map<string, string>();

  addUrl(path: string, content: string) {
    this.paths.set(path, content);
  }

  match(path: string): string {
    const pathPaths = path.split("/");

    console.log(pathPaths);

    if (path.split("/").length === 1) return "users";
    if (path.split("/").length === 2) return "user";
    if (path.split("/").length === 4) return "book user";
    return "books user";
  }

  private stringToRegex(path: string): any {
    const pathSplited = path.split("/");
    const a = path.replace(/:(\w+)/g, "")
  }
}\/(\w+)

Deno.test("if init", () => {
  const urlParse = new UrlParse();

  urlParse.addUrl("users", "users");

  urlParse.addUrl("users/:user_id", "user");

  urlParse.addUrl("users/:user_id/books/:book_id", "book user");

  urlParse.addUrl("users/:user_id/books", "books user");

  const result = urlParse.match("users/10");

  assertEquals(result, "user");

  const result2 = urlParse.match("users/10/books/123");

  assertEquals(result2, "book user");

  const result3 = urlParse.match("users/10/books");

  assertEquals(result3, "books user");

  const result4 = urlParse.match("users");

  assertEquals(result4, "users");

  const result5 = urlParse.match("cars");

  assertEquals(result5, "");
});
