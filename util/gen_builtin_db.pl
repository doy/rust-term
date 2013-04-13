#!/usr/bin/env perl
use strict;
use warnings;

my @terms = qw(
    linux
    xterm
    xterm-256color
    screen
    screen-256color
    rxvt
    rxvt-unicode
    aterm
    Eterm
    kterm
    gnome
);

my @caps = qw(
    clear
    setaf
    setab
    op
    sgr0
    home
    cup
    smul
    rmul
    smso
    rmso
    rev
    bold
    blink
    civis
    cnorm
    smcup
    rmcup
    smkx
    rmkx
    kbs
    cr
    ht
    kcuu1
    kcud1
    kcub1
    kcuf1
    khome
    kend
    kich1
    kdch1
    kf1
    kf2
    kf3
    kf4
    kf5
    kf6
    kf7
    kf8
    kf9
    kf10
    kf11
    kf12
);

my @db;
for my $term (@terms) {
    my @vals;
    for my $cap (@caps) {
        my $str = `tput -T$term $cap`;
        if ($? == 0) {
            push @vals, $str;
        }
        else {
            push @vals, undef;
        }
    }
    push @db, \@vals;
}

my $i = 0;
for my $row (@db) {
    print "    [ // " . $terms[$i] . "\n";
    my $j = 0;
    for my $val (@$row) {
        print "        ";
        if (defined $val) {
            $val =~ s/(\P{Print})/"\\x" . sprintf("%02x", ord($1))/ge;
            print "Some(\"$val\"),";
        }
        else {
            print "None,";
        }
        print " // " . $caps[$j] . "\n";
        $j++;
    }
    print "    ],\n";
    $i++;
}
