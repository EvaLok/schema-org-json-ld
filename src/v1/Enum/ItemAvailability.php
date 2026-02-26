<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Enum;

enum ItemAvailability: string {
	case InStock = 'https://schema.org/InStock';
	case OutOfStock = 'https://schema.org/OutOfStock';
	case Discontinued = 'https://schema.org/Discontinued';
}
