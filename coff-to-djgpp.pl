#!/usr/bin/perl -p0777
# Usage: coff-to-djgpp.pl < windows-coff.o > djgpp-coff.o
#
# renames .rdata section to .text, changes relative relocation format

# get coff header
my($magic, $nsec, $tim, $sym, $nsym, $optsz) = unpack("vvVVVv", $_);
# loop over all sections
for ($i = 0; $i < $nsec; $i++) {
  $ofs = 20 + $optsz + $i * 40;
  # rename .rdata to .text
  if (substr($_, $ofs, 8) eq ".rdata\0\0") {
    $_ = substr($_, 0, $ofs).
    ".text\0\0\0".substr($_, $ofs + 8)
  }
  # get section header
  my($n1, $n2, $vsize, $rva, $size, $data, $reloc, $lin, $nreloc) = unpack("VVVVVVVVv", substr($_, $ofs, 40));
  # loop over all relocations
  for ($j = 0; $j < $nreloc; $j++) {
    $ro = $reloc + $j * 10;
    # get relocation
    my($adr, $idx, $tp) = unpack("VVv", substr($_, $ro, 10));
    if ($tp == 20) { # relative relocation?
      # adjust original value of relative relocation to -address
      my $o = $data + $adr;
      my($v) = unpack("V", substr($_, $o, 4));
      $_ = substr($_, 0, $o).pack("V", -($adr + 4)).substr($_, $o + 4)
    }
  }
}
