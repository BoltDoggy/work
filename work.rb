# Homebrew formula for work CLI tool
# Install with: brew install BoltDoggy/work/work

class Work < Formula
  desc "A CLI tool to simplify Git worktree management"
  homepage "https://github.com/BoltDoggy/work"
  url "https://github.com/BoltDoggy/work.git", tag: "v0.1.0", revision: "main"
  license "MIT"

  depends_on "rust"

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/work", "--version"
    system "#{bin}/work", "--help"
  end
end
