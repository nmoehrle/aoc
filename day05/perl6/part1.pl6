my $num = 0;
for '../input.txt'.IO.words -> $word {
    next unless $word ~~ /(<[aeiou]>.*) ** 3..*/;
    next unless $word ~~ /(.)$0/;
    next if $word ~~ /'ab'|'cd'|'pq'|'xy'/;
    ++$num;
}
say $num;

