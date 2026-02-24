<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class NutritionInformation extends TypedSchema {
	public const A_SCHEMA_TYPE = 'NutritionInformation';

	public function __construct(
		public null|string $calories = null,
		public null|string $fatContent = null,
		public null|string $saturatedFatContent = null,
		public null|string $cholesterolContent = null,
		public null|string $sodiumContent = null,
		public null|string $carbohydrateContent = null,
		public null|string $fiberContent = null,
		public null|string $sugarContent = null,
		public null|string $proteinContent = null,
		public null|string $servingSize = null,
	) {}
}
