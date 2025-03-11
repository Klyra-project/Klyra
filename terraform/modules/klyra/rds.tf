resource "aws_db_subnet_group" "managed" {
  name        = "klyra_rds"
  description = "Subnet for RDS instances managed by klyra"
  subnet_ids  = [aws_subnet.backend_a.id, aws_subnet.backend_b.id]
}
