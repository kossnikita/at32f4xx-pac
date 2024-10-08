#[doc = "Register `CLR2` reader"]
pub type R = crate::R<CLR2_SPEC>;
#[doc = "Register `CLR2` writer"]
pub type W = crate::W<CLR2_SPEC>;
#[doc = "Field `FERRFC5` reader - Stream 5 clear FIFO error interrupt flag"]
pub type FERRFC5_R = crate::BitReader;
#[doc = "Field `FERRFC5` writer - Stream 5 clear FIFO error interrupt flag"]
pub type FERRFC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMERRFC5` reader - Stream 5 clear direct mode error interrupt flag"]
pub type DMERRFC5_R = crate::BitReader;
#[doc = "Field `DMERRFC5` writer - Stream 5 clear direct mode error interrupt flag"]
pub type DMERRFC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC5` reader - Stream 5 clear transfer error interrupt flag"]
pub type DTERRFC5_R = crate::BitReader;
#[doc = "Field `DTERRFC5` writer - Stream 5 clear transfer error interrupt flag"]
pub type DTERRFC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC5` reader - Stream 5 clear half data transfer interrupt flag"]
pub type HDTFC5_R = crate::BitReader;
#[doc = "Field `HDTFC5` writer - Stream 5 clear half data transfer interrupt flag"]
pub type HDTFC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC5` reader - Stream 5 clear full data transfer complete interrupt flag"]
pub type FDTFC5_R = crate::BitReader;
#[doc = "Field `FDTFC5` writer - Stream 5 clear full data transfer complete interrupt flag"]
pub type FDTFC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERRFC6` reader - Stream 6 clear FIFO error interrupt flag"]
pub type FERRFC6_R = crate::BitReader;
#[doc = "Field `FERRFC6` writer - Stream 6 clear FIFO error interrupt flag"]
pub type FERRFC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMERRFC6` reader - Stream 6 clear direct mode error interrupt flag"]
pub type DMERRFC6_R = crate::BitReader;
#[doc = "Field `DMERRFC6` writer - Stream 6 clear direct mode error interrupt flag"]
pub type DMERRFC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC6` reader - Stream 6 clear transfer error interrupt flag"]
pub type DTERRFC6_R = crate::BitReader;
#[doc = "Field `DTERRFC6` writer - Stream 6 clear transfer error interrupt flag"]
pub type DTERRFC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC6` reader - Stream 6 clear half data transfer interrupt flag"]
pub type HDTFC6_R = crate::BitReader;
#[doc = "Field `HDTFC6` writer - Stream 6 clear half data transfer interrupt flag"]
pub type HDTFC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC6` reader - Stream 6 clear full data transfer complete interrupt flag"]
pub type FDTFC6_R = crate::BitReader;
#[doc = "Field `FDTFC6` writer - Stream 6 clear full data transfer complete interrupt flag"]
pub type FDTFC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERRFC7` reader - Stream 7 clear FIFO error interrupt flag"]
pub type FERRFC7_R = crate::BitReader;
#[doc = "Field `FERRFC7` writer - Stream 7 clear FIFO error interrupt flag"]
pub type FERRFC7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMERRFC7` reader - Stream 7 clear direct mode error interrupt flag"]
pub type DMERRFC7_R = crate::BitReader;
#[doc = "Field `DMERRFC7` writer - Stream 7 clear direct mode error interrupt flag"]
pub type DMERRFC7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC7` reader - Stream 7 clear transfer error interrupt flag"]
pub type DTERRFC7_R = crate::BitReader;
#[doc = "Field `DTERRFC7` writer - Stream 7 clear transfer error interrupt flag"]
pub type DTERRFC7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC7` reader - Stream 7 clear half data transfer interrupt flag"]
pub type HDTFC7_R = crate::BitReader;
#[doc = "Field `HDTFC7` writer - Stream 7 clear half data transfer interrupt flag"]
pub type HDTFC7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC7` reader - Stream 7 clear full data transfer complete interrupt flag"]
pub type FDTFC7_R = crate::BitReader;
#[doc = "Field `FDTFC7` writer - Stream 7 clear full data transfer complete interrupt flag"]
pub type FDTFC7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERRFC8` reader - Stream 8 clear FIFO error interrupt flag"]
pub type FERRFC8_R = crate::BitReader;
#[doc = "Field `FERRFC8` writer - Stream 8 clear FIFO error interrupt flag"]
pub type FERRFC8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMERRFC8` reader - Stream 8 clear direct mode error interrupt flag"]
pub type DMERRFC8_R = crate::BitReader;
#[doc = "Field `DMERRFC8` writer - Stream 8 clear direct mode error interrupt flag"]
pub type DMERRFC8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC8` reader - Stream 8 clear transfer error interrupt flag"]
pub type DTERRFC8_R = crate::BitReader;
#[doc = "Field `DTERRFC8` writer - Stream 8 clear transfer error interrupt flag"]
pub type DTERRFC8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC8` reader - Stream 8 clear half data transfer interrupt flag"]
pub type HDTFC8_R = crate::BitReader;
#[doc = "Field `HDTFC8` writer - Stream 8 clear half data transfer interrupt flag"]
pub type HDTFC8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC8` reader - Stream 8 clear full data transfer complete interrupt flag"]
pub type FDTFC8_R = crate::BitReader;
#[doc = "Field `FDTFC8` writer - Stream 8 clear full data transfer complete interrupt flag"]
pub type FDTFC8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stream 5 clear FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrfc5(&self) -> FERRFC5_R {
        FERRFC5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Stream 5 clear direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrfc5(&self) -> DMERRFC5_R {
        DMERRFC5_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream 5 clear transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrfc5(&self) -> DTERRFC5_R {
        DTERRFC5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream 5 clear half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtfc5(&self) -> HDTFC5_R {
        HDTFC5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream 5 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    pub fn fdtfc5(&self) -> FDTFC5_R {
        FDTFC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream 6 clear FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrfc6(&self) -> FERRFC6_R {
        FERRFC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Stream 6 clear direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrfc6(&self) -> DMERRFC6_R {
        DMERRFC6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stream 6 clear transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrfc6(&self) -> DTERRFC6_R {
        DTERRFC6_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stream 6 clear half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtfc6(&self) -> HDTFC6_R {
        HDTFC6_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stream 6 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    pub fn fdtfc6(&self) -> FDTFC6_R {
        FDTFC6_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Stream 7 clear FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrfc7(&self) -> FERRFC7_R {
        FERRFC7_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Stream 7 clear direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrfc7(&self) -> DMERRFC7_R {
        DMERRFC7_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Stream 7 clear transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrfc7(&self) -> DTERRFC7_R {
        DTERRFC7_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stream 7 clear half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtfc7(&self) -> HDTFC7_R {
        HDTFC7_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Stream 7 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    pub fn fdtfc7(&self) -> FDTFC7_R {
        FDTFC7_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Stream 8 clear FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrfc8(&self) -> FERRFC8_R {
        FERRFC8_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Stream 8 clear direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrfc8(&self) -> DMERRFC8_R {
        DMERRFC8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Stream 8 clear transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrfc8(&self) -> DTERRFC8_R {
        DTERRFC8_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Stream 8 clear half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtfc8(&self) -> HDTFC8_R {
        HDTFC8_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Stream 8 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    pub fn fdtfc8(&self) -> FDTFC8_R {
        FDTFC8_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLR2")
            .field("fdtfc8", &self.fdtfc8())
            .field("hdtfc8", &self.hdtfc8())
            .field("dterrfc8", &self.dterrfc8())
            .field("dmerrfc8", &self.dmerrfc8())
            .field("ferrfc8", &self.ferrfc8())
            .field("fdtfc7", &self.fdtfc7())
            .field("hdtfc7", &self.hdtfc7())
            .field("dterrfc7", &self.dterrfc7())
            .field("dmerrfc7", &self.dmerrfc7())
            .field("ferrfc7", &self.ferrfc7())
            .field("fdtfc6", &self.fdtfc6())
            .field("hdtfc6", &self.hdtfc6())
            .field("dterrfc6", &self.dterrfc6())
            .field("dmerrfc6", &self.dmerrfc6())
            .field("ferrfc6", &self.ferrfc6())
            .field("fdtfc5", &self.fdtfc5())
            .field("hdtfc5", &self.hdtfc5())
            .field("dterrfc5", &self.dterrfc5())
            .field("dmerrfc5", &self.dmerrfc5())
            .field("ferrfc5", &self.ferrfc5())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stream 5 clear FIFO error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferrfc5(&mut self) -> FERRFC5_W<CLR2_SPEC> {
        FERRFC5_W::new(self, 0)
    }
    #[doc = "Bit 2 - Stream 5 clear direct mode error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmerrfc5(&mut self) -> DMERRFC5_W<CLR2_SPEC> {
        DMERRFC5_W::new(self, 2)
    }
    #[doc = "Bit 3 - Stream 5 clear transfer error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dterrfc5(&mut self) -> DTERRFC5_W<CLR2_SPEC> {
        DTERRFC5_W::new(self, 3)
    }
    #[doc = "Bit 4 - Stream 5 clear half data transfer interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hdtfc5(&mut self) -> HDTFC5_W<CLR2_SPEC> {
        HDTFC5_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stream 5 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn fdtfc5(&mut self) -> FDTFC5_W<CLR2_SPEC> {
        FDTFC5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Stream 6 clear FIFO error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferrfc6(&mut self) -> FERRFC6_W<CLR2_SPEC> {
        FERRFC6_W::new(self, 6)
    }
    #[doc = "Bit 8 - Stream 6 clear direct mode error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmerrfc6(&mut self) -> DMERRFC6_W<CLR2_SPEC> {
        DMERRFC6_W::new(self, 8)
    }
    #[doc = "Bit 9 - Stream 6 clear transfer error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dterrfc6(&mut self) -> DTERRFC6_W<CLR2_SPEC> {
        DTERRFC6_W::new(self, 9)
    }
    #[doc = "Bit 10 - Stream 6 clear half data transfer interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hdtfc6(&mut self) -> HDTFC6_W<CLR2_SPEC> {
        HDTFC6_W::new(self, 10)
    }
    #[doc = "Bit 11 - Stream 6 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn fdtfc6(&mut self) -> FDTFC6_W<CLR2_SPEC> {
        FDTFC6_W::new(self, 11)
    }
    #[doc = "Bit 16 - Stream 7 clear FIFO error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferrfc7(&mut self) -> FERRFC7_W<CLR2_SPEC> {
        FERRFC7_W::new(self, 16)
    }
    #[doc = "Bit 18 - Stream 7 clear direct mode error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmerrfc7(&mut self) -> DMERRFC7_W<CLR2_SPEC> {
        DMERRFC7_W::new(self, 18)
    }
    #[doc = "Bit 19 - Stream 7 clear transfer error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dterrfc7(&mut self) -> DTERRFC7_W<CLR2_SPEC> {
        DTERRFC7_W::new(self, 19)
    }
    #[doc = "Bit 20 - Stream 7 clear half data transfer interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hdtfc7(&mut self) -> HDTFC7_W<CLR2_SPEC> {
        HDTFC7_W::new(self, 20)
    }
    #[doc = "Bit 21 - Stream 7 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn fdtfc7(&mut self) -> FDTFC7_W<CLR2_SPEC> {
        FDTFC7_W::new(self, 21)
    }
    #[doc = "Bit 22 - Stream 8 clear FIFO error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferrfc8(&mut self) -> FERRFC8_W<CLR2_SPEC> {
        FERRFC8_W::new(self, 22)
    }
    #[doc = "Bit 24 - Stream 8 clear direct mode error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmerrfc8(&mut self) -> DMERRFC8_W<CLR2_SPEC> {
        DMERRFC8_W::new(self, 24)
    }
    #[doc = "Bit 25 - Stream 8 clear transfer error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dterrfc8(&mut self) -> DTERRFC8_W<CLR2_SPEC> {
        DTERRFC8_W::new(self, 25)
    }
    #[doc = "Bit 26 - Stream 8 clear half data transfer interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hdtfc8(&mut self) -> HDTFC8_W<CLR2_SPEC> {
        HDTFC8_W::new(self, 26)
    }
    #[doc = "Bit 27 - Stream 8 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn fdtfc8(&mut self) -> FDTFC8_W<CLR2_SPEC> {
        FDTFC8_W::new(self, 27)
    }
}
#[doc = "Interrupt flag clear register2\n\nYou can [`read`](crate::Reg::read) this register and get [`clr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR2_SPEC;
impl crate::RegisterSpec for CLR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr2::R`](R) reader structure"]
impl crate::Readable for CLR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clr2::W`](W) writer structure"]
impl crate::Writable for CLR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR2 to value 0"]
impl crate::Resettable for CLR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
