<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class MathSolver extends TypedSchema {
	public const A_SCHEMA_TYPE = ['MathSolver', 'LearningResource'];

	public function __construct(
		public string $url,
		public string $usageInfo,
		/** @var SolveMathAction|SolveMathAction[] $potentialAction */
		public SolveMathAction|array $potentialAction,
		public null|string $name = null,
		public null|string $inLanguage = null,
		public null|string $learningResourceType = null,
		/** @var string[]|null $assesses */
		public null|array $assesses = null,
	) {}
}
