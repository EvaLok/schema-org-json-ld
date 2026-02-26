<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

enum ReturnLabelSourceEnumeration: string {
	case ReturnLabelCustomerResponsibility = 'https://schema.org/ReturnLabelCustomerResponsibility';
	case ReturnLabelDownloadAndPrint = 'https://schema.org/ReturnLabelDownloadAndPrint';
	case ReturnLabelInBox = 'https://schema.org/ReturnLabelInBox';
}
