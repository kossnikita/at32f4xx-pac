#[doc = "Register `FRF` reader"]
pub type R = crate::R<FRF_SPEC>;
#[doc = "Register `FRF` writer"]
pub type W = crate::W<FRF_SPEC>;
#[doc = "Field `FRFSEL0` reader - Filter relation FIFO select"]
pub type FRFSEL0_R = crate::BitReader;
#[doc = "Field `FRFSEL0` writer - Filter relation FIFO select"]
pub type FRFSEL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRFSEL1` reader - Filter relation FIFO select"]
pub type FRFSEL1_R = crate::BitReader;
#[doc = "Field `FRFSEL1` writer - Filter relation FIFO select"]
pub type FRFSEL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRFSEL2` reader - Filter relation FIFO select"]
pub type FRFSEL2_R = crate::BitReader;
#[doc = "Field `FRFSEL2` writer - Filter relation FIFO select"]
pub type FRFSEL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRFSEL3` reader - Filter relation FIFO select"]
pub type FRFSEL3_R = crate::BitReader;
#[doc = "Field `FRFSEL3` writer - Filter relation FIFO select"]
pub type FRFSEL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRFSEL4` reader - Filter relation FIFO select"]
pub type FRFSEL4_R = crate::BitReader;
#[doc = "Field `FRFSEL4` writer - Filter relation FIFO select"]
pub type FRFSEL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRFSEL5` reader - Filter relation FIFO select"]
pub type FRFSEL5_R = crate::BitReader;
#[doc = "Field `FRFSEL5` writer - Filter relation FIFO select"]
pub type FRFSEL5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRFSEL6` reader - Filter relation FIFO select"]
pub type FRFSEL6_R = crate::BitReader;
#[doc = "Field `FRFSEL6` writer - Filter relation FIFO select"]
pub type FRFSEL6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRFSEL7` reader - Filter relation FIFO select"]
pub type FRFSEL7_R = crate::BitReader;
#[doc = "Field `FRFSEL7` writer - Filter relation FIFO select"]
pub type FRFSEL7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRFSEL8` reader - Filter relation FIFO select"]
pub type FRFSEL8_R = crate::BitReader;
#[doc = "Field `FRFSEL8` writer - Filter relation FIFO select"]
pub type FRFSEL8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRFSEL9` reader - Filter relation FIFO select"]
pub type FRFSEL9_R = crate::BitReader;
#[doc = "Field `FRFSEL9` writer - Filter relation FIFO select"]
pub type FRFSEL9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRFSEL10` reader - Filter relation FIFO select"]
pub type FRFSEL10_R = crate::BitReader;
#[doc = "Field `FRFSEL10` writer - Filter relation FIFO select"]
pub type FRFSEL10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRFSEL11` reader - Filter relation FIFO select"]
pub type FRFSEL11_R = crate::BitReader;
#[doc = "Field `FRFSEL11` writer - Filter relation FIFO select"]
pub type FRFSEL11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRFSEL12` reader - Filter relation FIFO select"]
pub type FRFSEL12_R = crate::BitReader;
#[doc = "Field `FRFSEL12` writer - Filter relation FIFO select"]
pub type FRFSEL12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRFSEL13` reader - Filter relation FIFO select"]
pub type FRFSEL13_R = crate::BitReader;
#[doc = "Field `FRFSEL13` writer - Filter relation FIFO select"]
pub type FRFSEL13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel0(&self) -> FRFSEL0_R {
        FRFSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel1(&self) -> FRFSEL1_R {
        FRFSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel2(&self) -> FRFSEL2_R {
        FRFSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel3(&self) -> FRFSEL3_R {
        FRFSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel4(&self) -> FRFSEL4_R {
        FRFSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel5(&self) -> FRFSEL5_R {
        FRFSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel6(&self) -> FRFSEL6_R {
        FRFSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel7(&self) -> FRFSEL7_R {
        FRFSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel8(&self) -> FRFSEL8_R {
        FRFSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel9(&self) -> FRFSEL9_R {
        FRFSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel10(&self) -> FRFSEL10_R {
        FRFSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel11(&self) -> FRFSEL11_R {
        FRFSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel12(&self) -> FRFSEL12_R {
        FRFSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel13(&self) -> FRFSEL13_R {
        FRFSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel0(&mut self) -> FRFSEL0_W<FRF_SPEC, 0> {
        FRFSEL0_W::new(self)
    }
    #[doc = "Bit 1 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel1(&mut self) -> FRFSEL1_W<FRF_SPEC, 1> {
        FRFSEL1_W::new(self)
    }
    #[doc = "Bit 2 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel2(&mut self) -> FRFSEL2_W<FRF_SPEC, 2> {
        FRFSEL2_W::new(self)
    }
    #[doc = "Bit 3 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel3(&mut self) -> FRFSEL3_W<FRF_SPEC, 3> {
        FRFSEL3_W::new(self)
    }
    #[doc = "Bit 4 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel4(&mut self) -> FRFSEL4_W<FRF_SPEC, 4> {
        FRFSEL4_W::new(self)
    }
    #[doc = "Bit 5 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel5(&mut self) -> FRFSEL5_W<FRF_SPEC, 5> {
        FRFSEL5_W::new(self)
    }
    #[doc = "Bit 6 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel6(&mut self) -> FRFSEL6_W<FRF_SPEC, 6> {
        FRFSEL6_W::new(self)
    }
    #[doc = "Bit 7 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel7(&mut self) -> FRFSEL7_W<FRF_SPEC, 7> {
        FRFSEL7_W::new(self)
    }
    #[doc = "Bit 8 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel8(&mut self) -> FRFSEL8_W<FRF_SPEC, 8> {
        FRFSEL8_W::new(self)
    }
    #[doc = "Bit 9 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel9(&mut self) -> FRFSEL9_W<FRF_SPEC, 9> {
        FRFSEL9_W::new(self)
    }
    #[doc = "Bit 10 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel10(&mut self) -> FRFSEL10_W<FRF_SPEC, 10> {
        FRFSEL10_W::new(self)
    }
    #[doc = "Bit 11 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel11(&mut self) -> FRFSEL11_W<FRF_SPEC, 11> {
        FRFSEL11_W::new(self)
    }
    #[doc = "Bit 12 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel12(&mut self) -> FRFSEL12_W<FRF_SPEC, 12> {
        FRFSEL12_W::new(self)
    }
    #[doc = "Bit 13 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel13(&mut self) -> FRFSEL13_W<FRF_SPEC, 13> {
        FRFSEL13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Filter related FIFO register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRF_SPEC;
impl crate::RegisterSpec for FRF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frf::R`](R) reader structure"]
impl crate::Readable for FRF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frf::W`](W) writer structure"]
impl crate::Writable for FRF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRF to value 0"]
impl crate::Resettable for FRF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
