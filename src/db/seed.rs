use sqlx::PgPool;

/// Insert 5 sample shipments from workbook1.xlsx rows 5-9 if the table is empty.
pub async fn seed_if_empty(pool: &PgPool) -> Result<(), sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM shipments")
        .fetch_one(pool)
        .await?;

    if count.0 > 0 {
        log::info!("Database already has {} shipments, skipping seed", count.0);
        return Ok(());
    }

    log::info!("Seeding 5 sample shipments from workbook1.xlsx");

    // Seed shipments one at a time
    for sql in SEED_SHIPMENTS {
        sqlx::query(sql).execute(pool).await?;
    }

    log::info!("Seed data inserted");
    Ok(())
}

const SEED_SHIPMENTS: &[&str] = &[
    r#"INSERT INTO shipments (
        shipment_ref, status,
        sc_po_id, sc_po_date, sc_po_by, buyer_name,
        booking_number, shipping_line, origin_port, warehouse_loc, loading_plan,
        shipper_name, consignee_name, etd,
        invoice_number, invoice_date, total_value_usd, drafts_date, bill_of_lading,
        customs_date, customs_number, customs_status
    ) VALUES (
        'SHP-2026-001', 'DRAFT',
        '1', '2026-06-15', 'Tuan', 'Element',
        'YMLAN6546547123', 'Yang Ming', 'Haiphong', 'Phu Cu', '2 K30, 2 M20 … mix 1K30 + 1M30',
        'Hung Phat', 'Kotor', '2026-06-28',
        'Inv 50-1', '2026-06-25', 333324, '2026-07-05', 'YSD354354',
        '2026-06-24', 'TKHQ364545', 'Green'
    )"#,
    r#"INSERT INTO shipments (
        shipment_ref, status,
        sc_po_id, sc_po_date, sc_po_by, buyer_name,
        booking_number, shipping_line, origin_port, warehouse_loc, loading_plan,
        shipper_name, consignee_name, etd,
        invoice_number, invoice_date, total_value_usd, drafts_date, bill_of_lading,
        customs_date, customs_number, customs_status
    ) VALUES (
        'SHP-2026-002', 'DRAFT',
        '2', '2026-06-18', 'Tuan', 'Element',
        'YMLAN6546547159', 'Yang Ming', 'Haiphong', 'Quang Chau', '5 M20',
        'Hung Phat', 'Kotor', '2026-06-30',
        'Inv 50-2', '2026-06-25', 154610, '2026-07-05', 'YSL565434',
        '2026-06-24', 'TKHQ364488', 'Green'
    )"#,
    r#"INSERT INTO shipments (
        shipment_ref, status,
        sc_po_id, sc_po_date, sc_po_by, buyer_name,
        booking_number, shipping_line, origin_port, warehouse_loc, loading_plan,
        shipper_name, consignee_name, etd,
        invoice_number, invoice_date, total_value_usd, drafts_date, bill_of_lading,
        customs_date, customs_number, customs_status
    ) VALUES (
        'SHP-2026-003', 'DRAFT',
        '3', '2026-06-20', 'Hung', 'UAE',
        'MKLAN6546547436', 'Maersk', 'Haiphong', 'Nam Dinh', '4K 20 + 4M20',
        'An Dien', 'Global Trade', '2026-07-03',
        'Inv 16', '2026-06-27', 132154, '2026-07-06', 'BN343843',
        '2026-06-24', 'TKHQ344354', 'Green'
    )"#,
    r#"INSERT INTO shipments (
        shipment_ref, status,
        sc_po_id, sc_po_date, sc_po_by, buyer_name,
        booking_number, shipping_line, origin_port, warehouse_loc, loading_plan,
        shipper_name, consignee_name, etd,
        invoice_number, invoice_date, total_value_usd, drafts_date, bill_of_lading,
        customs_date, customs_number, customs_status
    ) VALUES (
        'SHP-2026-004', 'DRAFT',
        '4', '2026-06-21', 'Hue', 'Golden Banyan',
        'KLJAN6546543574', 'APL', 'Haiphong', 'Nam Dinh', '1K - A10',
        'An Dien', 'UK Trading Holding', '2026-07-03',
        'Inv 11', '2026-06-28', 123466, '2026-07-07', 'KT3365434',
        '2026-06-24', 'TKHQ768765', 'Green'
    )"#,
    r#"INSERT INTO shipments (
        shipment_ref, status,
        sc_po_id, sc_po_date, sc_po_by, buyer_name,
        booking_number, shipping_line, origin_port, warehouse_loc, loading_plan,
        shipper_name, consignee_name, etd,
        invoice_number, invoice_date, total_value_usd, drafts_date, bill_of_lading,
        customs_date, customs_number, customs_status
    ) VALUES (
        'SHP-2026-005', 'DRAFT',
        '5', '2026-06-21', 'Minh', 'Barko',
        'HUBAN6546547987', 'Evergreen', 'Haiphong', 'Quang Chau', '3 K20',
        'Hung Phat', 'Postera', '2026-07-03',
        'Inv 33', '2026-06-28', 12347, '2026-07-07', 'HHH354736',
        '2026-06-24', 'TKHQ243543', 'Green'
    )"#,
];

// ponytail: SEED_CUSTOMERS + SEED_CARRIERS removed — tables dropped in schema.
