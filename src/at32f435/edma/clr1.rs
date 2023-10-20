#[doc = "Register `CLR1` reader"]
pub type R = crate::R<CLR1_SPEC>;
#[doc = "Register `CLR1` writer"]
pub type W = crate::W<CLR1_SPEC>;
#[doc = "Field `FERRFC1` reader - Stream 1 clear FIFO error interrupt flag"]
pub type FERRFC1_R = crate::BitReader;
#[doc = "Field `FERRFC1` writer - Stream 1 clear FIFO error interrupt flag"]
pub type FERRFC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMERRFC1` reader - Stream 1 clear direct mode error interrupt flag"]
pub type DMERRFC1_R = crate::BitReader;
#[doc = "Field `DMERRFC1` writer - Stream 1 clear direct mode error interrupt flag"]
pub type DMERRFC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC1` reader - Stream 1 clear transfer error interrupt flag"]
pub type DTERRFC1_R = crate::BitReader;
#[doc = "Field `DTERRFC1` writer - Stream 1 clear transfer error interrupt flag"]
pub type DTERRFC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC1` reader - Stream 1 clear half data transfer interrupt flag"]
pub type HDTFC1_R = crate::BitReader;
#[doc = "Field `HDTFC1` writer - Stream 1 clear half data transfer interrupt flag"]
pub type HDTFC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC1` reader - Stream 1 clear full data transfer complete interrupt flag"]
pub type FDTFC1_R = crate::BitReader;
#[doc = "Field `FDTFC1` writer - Stream 1 clear full data transfer complete interrupt flag"]
pub type FDTFC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERRFC2` reader - Stream 2 clear FIFO error interrupt flag"]
pub type FERRFC2_R = crate::BitReader;
#[doc = "Field `FERRFC2` writer - Stream 2 clear FIFO error interrupt flag"]
pub type FERRFC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMERRFC2` reader - Stream 2 clear direct mode error interrupt flag"]
pub type DMERRFC2_R = crate::BitReader;
#[doc = "Field `DMERRFC2` writer - Stream 2 clear direct mode error interrupt flag"]
pub type DMERRFC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC2` reader - Stream 2 clear transfer error interrupt flag"]
pub type DTERRFC2_R = crate::BitReader;
#[doc = "Field `DTERRFC2` writer - Stream 2 clear transfer error interrupt flag"]
pub type DTERRFC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC2` reader - Stream 2 clear half data transfer interrupt flag"]
pub type HDTFC2_R = crate::BitReader;
#[doc = "Field `HDTFC2` writer - Stream 2 clear half data transfer interrupt flag"]
pub type HDTFC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC2` reader - Stream 2 clear full data transfer complete interrupt flag"]
pub type FDTFC2_R = crate::BitReader;
#[doc = "Field `FDTFC2` writer - Stream 2 clear full data transfer complete interrupt flag"]
pub type FDTFC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERRFC3` reader - Stream 3 clear FIFO error interrupt flag"]
pub type FERRFC3_R = crate::BitReader;
#[doc = "Field `FERRFC3` writer - Stream 3 clear FIFO error interrupt flag"]
pub type FERRFC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMERRFC3` reader - Stream 3 clear direct mode error interrupt flag"]
pub type DMERRFC3_R = crate::BitReader;
#[doc = "Field `DMERRFC3` writer - Stream 3 clear direct mode error interrupt flag"]
pub type DMERRFC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC3` reader - Stream 3 clear transfer error interrupt flag"]
pub type DTERRFC3_R = crate::BitReader;
#[doc = "Field `DTERRFC3` writer - Stream 3 clear transfer error interrupt flag"]
pub type DTERRFC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC3` reader - Stream 3 clear half data transfer interrupt flag"]
pub type HDTFC3_R = crate::BitReader;
#[doc = "Field `HDTFC3` writer - Stream 3 clear half data transfer interrupt flag"]
pub type HDTFC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC3` reader - Stream 3 clear full data transfer complete interrupt flag"]
pub type FDTFC3_R = crate::BitReader;
#[doc = "Field `FDTFC3` writer - Stream 3 clear full data transfer complete interrupt flag"]
pub type FDTFC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERRFC4` reader - Stream 4 clear FIFO error interrupt flag"]
pub type FERRFC4_R = crate::BitReader;
#[doc = "Field `FERRFC4` writer - Stream 4 clear FIFO error interrupt flag"]
pub type FERRFC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMERRFC4` reader - Stream 4 clear direct mode error interrupt flag"]
pub type DMERRFC4_R = crate::BitReader;
#[doc = "Field `DMERRFC4` writer - Stream 4 clear direct mode error interrupt flag"]
pub type DMERRFC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC4` reader - Stream 4 clear transfer error interrupt flag"]
pub type DTERRFC4_R = crate::BitReader;
#[doc = "Field `DTERRFC4` writer - Stream 4 clear transfer error interrupt flag"]
pub type DTERRFC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC4` reader - Stream 4 clear half data transfer interrupt flag"]
pub type HDTFC4_R = crate::BitReader;
#[doc = "Field `HDTFC4` writer - Stream 4 clear half data transfer interrupt flag"]
pub type HDTFC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC4` reader - Stream 4 clear full data transfer complete interrupt flag"]
pub type FDTFC4_R = crate::BitReader;
#[doc = "Field `FDTFC4` writer - Stream 4 clear full data transfer complete interrupt flag"]
pub type FDTFC4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stream 1 clear FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrfc1(&self) -> FERRFC1_R {
        FERRFC1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Stream 1 clear direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrfc1(&self) -> DMERRFC1_R {
        DMERRFC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream 1 clear transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrfc1(&self) -> DTERRFC1_R {
        DTERRFC1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream 1 clear half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtfc1(&self) -> HDTFC1_R {
        HDTFC1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream 1 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    pub fn fdtfc1(&self) -> FDTFC1_R {
        FDTFC1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream 2 clear FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrfc2(&self) -> FERRFC2_R {
        FERRFC2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Stream 2 clear direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrfc2(&self) -> DMERRFC2_R {
        DMERRFC2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stream 2 clear transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrfc2(&self) -> DTERRFC2_R {
        DTERRFC2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stream 2 clear half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtfc2(&self) -> HDTFC2_R {
        HDTFC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stream 2 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    pub fn fdtfc2(&self) -> FDTFC2_R {
        FDTFC2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Stream 3 clear FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrfc3(&self) -> FERRFC3_R {
        FERRFC3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Stream 3 clear direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrfc3(&self) -> DMERRFC3_R {
        DMERRFC3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Stream 3 clear transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrfc3(&self) -> DTERRFC3_R {
        DTERRFC3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stream 3 clear half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtfc3(&self) -> HDTFC3_R {
        HDTFC3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Stream 3 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    pub fn fdtfc3(&self) -> FDTFC3_R {
        FDTFC3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Stream 4 clear FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrfc4(&self) -> FERRFC4_R {
        FERRFC4_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Stream 4 clear direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrfc4(&self) -> DMERRFC4_R {
        DMERRFC4_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Stream 4 clear transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrfc4(&self) -> DTERRFC4_R {
        DTERRFC4_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Stream 4 clear half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtfc4(&self) -> HDTFC4_R {
        HDTFC4_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Stream 4 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    pub fn fdtfc4(&self) -> FDTFC4_R {
        FDTFC4_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLR1")
            .field("fdtfc4", &format_args!("{}", self.fdtfc4().bit()))
            .field("hdtfc4", &format_args!("{}", self.hdtfc4().bit()))
            .field("dterrfc4", &format_args!("{}", self.dterrfc4().bit()))
            .field("dmerrfc4", &format_args!("{}", self.dmerrfc4().bit()))
            .field("ferrfc4", &format_args!("{}", self.ferrfc4().bit()))
            .field("fdtfc3", &format_args!("{}", self.fdtfc3().bit()))
            .field("hdtfc3", &format_args!("{}", self.hdtfc3().bit()))
            .field("dterrfc3", &format_args!("{}", self.dterrfc3().bit()))
            .field("dmerrfc3", &format_args!("{}", self.dmerrfc3().bit()))
            .field("ferrfc3", &format_args!("{}", self.ferrfc3().bit()))
            .field("fdtfc2", &format_args!("{}", self.fdtfc2().bit()))
            .field("hdtfc2", &format_args!("{}", self.hdtfc2().bit()))
            .field("dterrfc2", &format_args!("{}", self.dterrfc2().bit()))
            .field("dmerrfc2", &format_args!("{}", self.dmerrfc2().bit()))
            .field("ferrfc2", &format_args!("{}", self.ferrfc2().bit()))
            .field("fdtfc1", &format_args!("{}", self.fdtfc1().bit()))
            .field("hdtfc1", &format_args!("{}", self.hdtfc1().bit()))
            .field("dterrfc1", &format_args!("{}", self.dterrfc1().bit()))
            .field("dmerrfc1", &format_args!("{}", self.dmerrfc1().bit()))
            .field("ferrfc1", &format_args!("{}", self.ferrfc1().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CLR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Stream 1 clear FIFO error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferrfc1(&mut self) -> FERRFC1_W<CLR1_SPEC> {
        FERRFC1_W::new(self, 0)
    }
    #[doc = "Bit 2 - Stream 1 clear direct mode error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmerrfc1(&mut self) -> DMERRFC1_W<CLR1_SPEC> {
        DMERRFC1_W::new(self, 2)
    }
    #[doc = "Bit 3 - Stream 1 clear transfer error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dterrfc1(&mut self) -> DTERRFC1_W<CLR1_SPEC> {
        DTERRFC1_W::new(self, 3)
    }
    #[doc = "Bit 4 - Stream 1 clear half data transfer interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hdtfc1(&mut self) -> HDTFC1_W<CLR1_SPEC> {
        HDTFC1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stream 1 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn fdtfc1(&mut self) -> FDTFC1_W<CLR1_SPEC> {
        FDTFC1_W::new(self, 5)
    }
    #[doc = "Bit 6 - Stream 2 clear FIFO error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferrfc2(&mut self) -> FERRFC2_W<CLR1_SPEC> {
        FERRFC2_W::new(self, 6)
    }
    #[doc = "Bit 8 - Stream 2 clear direct mode error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmerrfc2(&mut self) -> DMERRFC2_W<CLR1_SPEC> {
        DMERRFC2_W::new(self, 8)
    }
    #[doc = "Bit 9 - Stream 2 clear transfer error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dterrfc2(&mut self) -> DTERRFC2_W<CLR1_SPEC> {
        DTERRFC2_W::new(self, 9)
    }
    #[doc = "Bit 10 - Stream 2 clear half data transfer interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hdtfc2(&mut self) -> HDTFC2_W<CLR1_SPEC> {
        HDTFC2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Stream 2 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn fdtfc2(&mut self) -> FDTFC2_W<CLR1_SPEC> {
        FDTFC2_W::new(self, 11)
    }
    #[doc = "Bit 16 - Stream 3 clear FIFO error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferrfc3(&mut self) -> FERRFC3_W<CLR1_SPEC> {
        FERRFC3_W::new(self, 16)
    }
    #[doc = "Bit 18 - Stream 3 clear direct mode error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmerrfc3(&mut self) -> DMERRFC3_W<CLR1_SPEC> {
        DMERRFC3_W::new(self, 18)
    }
    #[doc = "Bit 19 - Stream 3 clear transfer error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dterrfc3(&mut self) -> DTERRFC3_W<CLR1_SPEC> {
        DTERRFC3_W::new(self, 19)
    }
    #[doc = "Bit 20 - Stream 3 clear half data transfer interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hdtfc3(&mut self) -> HDTFC3_W<CLR1_SPEC> {
        HDTFC3_W::new(self, 20)
    }
    #[doc = "Bit 21 - Stream 3 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn fdtfc3(&mut self) -> FDTFC3_W<CLR1_SPEC> {
        FDTFC3_W::new(self, 21)
    }
    #[doc = "Bit 22 - Stream 4 clear FIFO error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferrfc4(&mut self) -> FERRFC4_W<CLR1_SPEC> {
        FERRFC4_W::new(self, 22)
    }
    #[doc = "Bit 24 - Stream 4 clear direct mode error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmerrfc4(&mut self) -> DMERRFC4_W<CLR1_SPEC> {
        DMERRFC4_W::new(self, 24)
    }
    #[doc = "Bit 25 - Stream 4 clear transfer error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dterrfc4(&mut self) -> DTERRFC4_W<CLR1_SPEC> {
        DTERRFC4_W::new(self, 25)
    }
    #[doc = "Bit 26 - Stream 4 clear half data transfer interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hdtfc4(&mut self) -> HDTFC4_W<CLR1_SPEC> {
        HDTFC4_W::new(self, 26)
    }
    #[doc = "Bit 27 - Stream 4 clear full data transfer complete interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn fdtfc4(&mut self) -> FDTFC4_W<CLR1_SPEC> {
        FDTFC4_W::new(self, 27)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt flag clear register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR1_SPEC;
impl crate::RegisterSpec for CLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr1::R`](R) reader structure"]
impl crate::Readable for CLR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clr1::W`](W) writer structure"]
impl crate::Writable for CLR1_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLR1 to value 0"]
impl crate::Resettable for CLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
