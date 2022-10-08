


f <- list.files(path = "./values", pattern = "*.txt$", full.names = TRUE)

plot_overlay <- function(p1, p2) {
  vals1 <- scan(p1, numeric())
  vals2 <- scan(p2, numeric())
  dens1 <- density(vals1)
  dens2 <- density(vals2)
  xlim <- range(dens1$x, dens2$x)
  ylim <- range(dens1$y, dens2$y)
  val1col <- rgb(1, 0, 0, 0.2)
  val2col <- rgb(0, 0, 1, 0.2)
  main <- gsub("./values/S[1-9](.*).txt", "S1 (M145) vs S2 (M1152) \\1 Distribution", p1)
  xlab <- gsub("./values/S[1-9](.*).txt", "\\1", p1)
  plot(dens1, xlim = xlim, ylim = ylim, xlab = xlab, main = main, panel.first = grid())
  polygon(dens1, density = -1, col = val1col)
  polygon(dens2, density = -1, col = val2col)
  legend("topleft", c(p1, p2), fill = c(val1col, val2col), bty = "n", border = NA)
}

plot_overlay2 <- function(p1, p2) {
  vals1 <- scan(p1, numeric())
  vals2 <- scan(p2, numeric())
  dens1 <- density(vals1)
  dens2 <- density(vals2)
  xlim <- range(dens1$x, dens2$x)
  ylim <- range(dens1$y, dens2$y)
  val1col <- rgb(1, 0, 0, 0.2)
  val2col <- rgb(0, 0, 1, 0.2)
  main <- gsub("./values/S[1-9](.*).txt", "S1 (M145) vs S2 (M1152) \\1 Distribution", p1)
  xlab <- gsub("./values/S[1-9](.*).txt", "\\1", p1)
  hist(vals1,
    xlim = xlim, col = val1col, border = FALSE, main = main, xlab = xlab,
    cex.lab = 1.5, cex.main = 1.5, cex.sub = 1.5, cex.axis = 1.5
  )
  hist(vals2,
    xlim = xlim, col = val2col, border = FALSE, add = TRUE, main = main,
    xlab = xlab, cex.lab = 1.5, cex.main = 1.5, cex.sub = 1.5, cex.axis = 1.5
  )
  legend("topright", c(p1, p2), fill = c(val1col, val2col), bty = "n", border = NA, cex = 1.5)
}


for (i in 1:6) {
  png(paste(f[i], "2.png", sep = ""), 600, 600)
  plot_overlay2(f[i], f[i + 6])
  dev.off()
}
