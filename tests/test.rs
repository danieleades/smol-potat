#[smol_potat::test]
async fn test() -> std::io::Result<()> {
    assert_eq!(2 * 2, 4);
    Ok(())
}
