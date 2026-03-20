class BeenThere < Formula
  desc "Terminal application for listing the countries you've visited with other interesting statistics"
  homepage "https://github.com/Wildhoney/BeenThere"
  version "${VERSION}"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Wildhoney/BeenThere/releases/download/v${VERSION}/been-there-aarch64-apple-darwin.tar.gz"
      sha256 "${SHA256_ARM}"
    else
      url "https://github.com/Wildhoney/BeenThere/releases/download/v${VERSION}/been-there-x86_64-apple-darwin.tar.gz"
      sha256 "${SHA256_X86}"
    end
  end

  on_linux do
    url "https://github.com/Wildhoney/BeenThere/releases/download/v${VERSION}/been-there-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "${SHA256_LINUX}"
  end

  def install
    bin.install "been-there"
    bin.install_symlink "been-there" => "bt"
  end

  test do
    assert_match "BeenThere", shell_output("#{bin}/been-there --help")
  end
end
