<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class InteractionCounter extends TypedSchema {
	public const A_SCHEMA_TYPE = 'InteractionCounter';

	public function __construct(
		public string $interactionType,
		public int $userInteractionCount,
		public null|string $interactionService = null,
	) {}
}
