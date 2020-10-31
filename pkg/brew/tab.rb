class BrrrrBin < Formula
  version '0.6.2'
  desc "Fast command line tool to process biological sequences and annotations to modern
file formats."
  homepage "https://github.com/tshauck/brrrr/"

  if OS.mac?
      url "https://github.com/brrrr/releases/download/#{version}/brrrr-#{version}-x86_64-apple-darwin"
      sha256 "bb22b8268211ebca106bb42acddd7b24e85a8da566a4013213eb6a8f34f59072"
  end

  def install
    bin.install "brrrr"
  end
end
