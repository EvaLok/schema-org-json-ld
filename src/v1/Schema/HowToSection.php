<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class HowToSection extends TypedSchema {
	public const A_SCHEMA_TYPE = 'HowToSection';

	public function __construct(
		public string $name,
		/** @var HowToStep[] $itemListElement */
		public array $itemListElement,
	) {}
}
