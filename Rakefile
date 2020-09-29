require "cgi"

task :default do
  codes = []
  i = 0
  define = ""
  invoke = ""
  state = nil
  stdout = ""

  (File.read("markup/examples/syntax.txt") + "\n").lines.each do |line|
    if line.strip.empty?
      stdout << line
      next if define.empty? || invoke.empty?
      codes << { define: define.strip, invoke: invoke.strip }
      define = ""
      invoke = ""
      stdout << "{{#{i}}}"
      i += 1
      state = nil
      next
    end

    if line.start_with?("+ ")
      state = :define
      define = line[2..-1]
      next
    end

    if line.start_with?("- ")
      state = :invoke
      invoke = line[2..-1]
      next
    end

    if state
      case state
      when :define then define << line
      when :invoke then invoke << line
      end
      next
    end

    stdout << line
  end

  rs = File.open("markup/tests/tests.rs", "wb")

  codes.each.with_index do |code, index|
    rs << <<-RS
mod e#{index + 1} {
    markup::define! {
#{indent(code[:define], 8)}
    }

    #[test] fn t() {
        insta::assert_display_snapshot!(#{code[:invoke]});
    }
}

RS
  end

  rs.close

  system "rustfmt markup/tests/tests.rs"

  rs = File.open("markup/examples/syntax.rs", "wb")

  codes.each.with_index do |code, index|
    rs << "mod e#{index} { markup::define! {\n#{indent(code[:define], 4)}\n} }\n\n"
  end

  rs << "fn main() {\n"
  codes.each.with_index do |code, index|
    rs << "    println!(\"{}\\n\", e#{index}::#{code[:invoke]});\n"
  end
  rs << "}\n"

  rs.close

  output = `cd markup/examples && cargo run --quiet --example syntax`.split("\n\n")

  rm "markup/examples/syntax.rs"

  md = stdout.gsub(/\n\{\{(\d+)\}\}/) {
    index = $1.to_i
    <<-MD
```rust
markup::define! {
#{indent codes[index][:define], 4}
}
```
```rust
println!("{}", #{codes[index][:invoke]});
```
```html
#{output[index].strip}
```

MD
  }

  before = "<!-- Syntax -->"
  after = "<!-- /Syntax -->"
  readme = File.read("README.md")
  new_readme =
    readme.lines.take_while { |line| line.strip != before }.join +
    before + "\n\n" + md +
    readme.lines.drop_while { |line| line.strip != after }.join
  File.write("README.md", new_readme)
end

def indent(string, by)
  string.lines.map do |line|
    (" " * by) + line
  end.join
end
