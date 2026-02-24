<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Quiz extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Quiz';

	public function __construct(
		/** @var Question[] $hasPart */
		public array $hasPart,
		public null|string $about = null,
		public null|AlignmentObject $educationalAlignment = null,
		public null|string $name = null,
		public null|string $description = null,
	) {}
}
