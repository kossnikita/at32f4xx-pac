#[doc = "Register `IDEN` reader"]
pub type R = crate::R<IDEN_SPEC>;
#[doc = "Register `IDEN` writer"]
pub type W = crate::W<IDEN_SPEC>;
#[doc = "Field `OVFIEN` reader - Overflow interrupt enable"]
pub type OVFIEN_R = crate::BitReader;
#[doc = "Field `OVFIEN` writer - Overflow interrupt enable"]
pub type OVFIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C1IEN` reader - Channel 1 interrupt enable"]
pub type C1IEN_R = crate::BitReader;
#[doc = "Field `C1IEN` writer - Channel 1 interrupt enable"]
pub type C1IEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C2IEN` reader - Channel 2 interrupt enable"]
pub type C2IEN_R = crate::BitReader;
#[doc = "Field `C2IEN` writer - Channel 2 interrupt enable"]
pub type C2IEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIEN` reader - Trigger interrupt enable"]
pub type TIEN_R = crate::BitReader;
#[doc = "Field `TIEN` writer - Trigger interrupt enable"]
pub type TIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&self) -> OVFIEN_R {
        OVFIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt enable"]
    #[inline(always)]
    pub fn c1ien(&self) -> C1IEN_R {
        C1IEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 interrupt enable"]
    #[inline(always)]
    pub fn c2ien(&self) -> C2IEN_R {
        C2IEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn tien(&self) -> TIEN_R {
        TIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfien(&mut self) -> OVFIEN_W<IDEN_SPEC, 0> {
        OVFIEN_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1ien(&mut self) -> C1IEN_W<IDEN_SPEC, 1> {
        C1IEN_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2ien(&mut self) -> C2IEN_W<IDEN_SPEC, 2> {
        C2IEN_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tien(&mut self) -> TIEN_W<IDEN_SPEC, 6> {
        TIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt/DMA enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iden::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iden::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDEN_SPEC;
impl crate::RegisterSpec for IDEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iden::R`](R) reader structure"]
impl crate::Readable for IDEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iden::W`](W) writer structure"]
impl crate::Writable for IDEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDEN to value 0"]
impl crate::Resettable for IDEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}