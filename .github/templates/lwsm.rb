class Lwsm < Formula
  desc "list & search files by word/sentence"
  homepage "https://github.com/K-Ryo-ta/LWSM"
  version "${VERSION}"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/K-Ryo-ta/LWSM/releases/download/v${VERSION}/lwsm-${VERSION}_arm64_darwin.tar.gz"
      sha256 "${SHA256_DARWIN_ARM64}"
    end
    on_intel do
      url "https://github.com/K-Ryo-ta/LWSM/releases/download/v${VERSION}/lwsm-${VERSION}_amd64_darwin.tar.gz"
      sha256 "${SHA256_DARWIN_AMD64}"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/K-Ryo-ta/LWSM/releases/download/v${VERSION}/lwsm-${VERSION}_arm64_linux.tar.gz"
      sha256 "${SHA256_LINUX_ARM64}"
    end
    on_intel do
      url "https://github.com/K-Ryo-ta/LWSM/releases/download/v${VERSION}/lwsm-${VERSION}_amd64_linux.tar.gz"
      sha256 "${SHA256_LINUX_AMD64}"
    end
  end

  def install
    bin.install "lwsm"

    # generate shell completions into the buildpath, then install them
    system bin/"lwsm", "--completions"
    bash_completion.install "completions/bash/lwsm" => "lwsm"
    zsh_completion.install "completions/zsh/_lwsm"
    fish_completion.install "completions/fish/lwsm" => "lwsm.fish"
  end

  test do
    system bin/"lwsm", "--help"
  end
end
