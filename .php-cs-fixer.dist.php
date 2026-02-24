<?php

$finder = (new PhpCsFixer\Finder())
    ->in(__DIR__ . '/src')
    ->in(__DIR__ . '/test')
;

return (new PhpCsFixer\Config())
    ->setRules([
        // Base ruleset
        '@PER-CS2.0' => true,

        // Match existing style: tabs
        'indentation_type' => true,

        // Opening brace on same line (existing style)
        'curly_braces_position' => [
            'classes_opening_brace' => 'same_line',
            'functions_opening_brace' => 'same_line',
        ],

        // Clean up whitespace
        'no_trailing_whitespace' => true,
        'no_whitespace_in_blank_line' => true,
        'single_blank_line_at_eof' => true,

        // Import ordering
        'ordered_imports' => ['sort_algorithm' => 'alpha'],
        'no_unused_imports' => true,

        // Array syntax
        'array_syntax' => ['syntax' => 'short'],

        // No strict_types — matches existing codebase
        'declare_strict_types' => false,
    ])
    ->setIndent("\t")
    ->setLineEnding("\n")
    ->setFinder($finder)
;
