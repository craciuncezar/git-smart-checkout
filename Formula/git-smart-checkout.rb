class Gitsmartcheckout < Formula
    desc "A git command extension for switching git branches more efficiently."
    homepage "https://github.com/craciuncezar/git-smart-checkout"
    url "https://github.com/craciuncezar/git-smart-checkout/releases/download/v0.1.0/git-smart-checkout.tar.gz"
    sha256 "81d1169cc14c86c1b33b7e9b3064c24ecf708da7a2a9438a2a400dac198213de"
    license "MIT"
    version "0.1.0"
    
    def install
      bin.install "git-smart-checkout"
    end
  end