my $num = 0;
for '../input.txt'.IO.words -> $word {
    next unless $word ~~ /(..).*$0/;
    next unless $word ~~ /(.).$0/;
    ++$num;
}
say $num;

