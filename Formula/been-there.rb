class BeenThere < Formula
  desc "Terminal application for listing the countries you've visited with other interesting statistics"
  homepage "https://github.com/Wildhoney/BeenThere"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Wildhoney/BeenThere/releases/download/v0.1.0/been-there-aarch64-apple-darwin.tar.gz"
      sha256 "d68f21a66fd1ef7eccb9793ef3f9a69c7bc3ec3e44334c1714868336a147ddec"
    else
      url "https://github.com/Wildhoney/BeenThere/releases/download/v0.1.0/been-there-x86_64-apple-darwin.tar.gz"
      sha256 "c6154a3ce4adbf319ecb17a87f0126609092384d68eea2d752031d2f385af960"
    end
  end

  on_linux do
    url "https://github.com/Wildhoney/BeenThere/releases/download/v0.1.0/been-there-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "9066c217ea821a4c55f7812dc243ed6f12b14f037507c5a91b51b83f2debae6d"
  end

  def install
    bin.install "been-there"
    bin.install_symlink "been-there" => "bt"
  end

  test do
    assert_match "BeenThere", shell_output("#{bin}/been-there --help")
  end
end
