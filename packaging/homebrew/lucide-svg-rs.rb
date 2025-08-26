class LucideOfflineCli < Formula
  desc 'Offline Lucide icons CLI and library'
  homepage 'https://github.com/soulcorrea/lucide-svg-rs'
  version '<VERSION>'
  url 'https://github.com/soulcorrea/lucide-svg-rs/releases/download/v<VERSION>/lucide-svg-rs-macos-x86_64.zip'
  sha256 '<SHA256>'
  def install
    bin.install 'lucide-svg-rs'
  end
end
