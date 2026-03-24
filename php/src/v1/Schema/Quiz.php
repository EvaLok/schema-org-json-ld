<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Quiz extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Quiz';

	public function __construct(
		/** @var Question[] $hasPart */
		public array $hasPart,
		public null|string|Thing $about = null,
		/** @var AlignmentObject|AlignmentObject[]|null */
		public null|AlignmentObject|array $educationalAlignment = null,
		public null|string $name = null,
		public null|string $description = null,
	) {}
}
