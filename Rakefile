task :default do
  md = File.read("docs/reference.md").gsub(/^#/, "###")

  codes = md.scan(/(```rust\n(.*?)\n```)/m)

  File.open("markup/examples/reference.rs", "w") do |f|
    f << "use std::io::Write;\n\n"
    codes.each.with_index do |code, i|
      f << "fn _#{i}() {\n#{code[1]}\n}\n\n"
    end
    f << "fn main() {"
    0.upto(codes.size - 1).each do |i|
      f << "_#{i}(); println!(\"---\");"
    end
    f << "}"
  end

  system "rustfmt markup/examples/reference.rs"

  output = `cargo run --example reference`.strip.split("---").map(&:strip)

  if codes.size != output.size
    raise "number of codes and outputs do not match"
  end

  codes.zip(output).each do |code, output|
    md = md.sub(code[0], "\
<table>
  <tr><th>Code</th></tr>
  <tr><td width=\"1000px\">

  ```rust
#{indent(code[1], with: "  ")}
  ```
  </td></tr>
  <tr><th>Output</th></tr>
  <tr><td width=\"1000px\">

  ```html
#{indent(output, with: "  ")}
  ```
  </td></tr>
</table>
")
  end

  puts md
end

def indent(string, with:)
  string.split("\n").map { |line| line.strip.empty? ? "" : "#{with}#{line}" }.join("\n")
end
