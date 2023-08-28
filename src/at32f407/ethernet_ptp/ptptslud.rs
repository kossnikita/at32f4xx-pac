#[doc = "Register `PTPTSLUD` reader"]
pub type R = crate::R<PTPTSLUD_SPEC>;
#[doc = "Register `PTPTSLUD` writer"]
pub type W = crate::W<PTPTSLUD_SPEC>;
#[doc = "Field `TSS` reader - Timestamp subseconds"]
pub type TSS_R = crate::FieldReader<u32>;
#[doc = "Field `TSS` writer - Timestamp subseconds"]
pub type TSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 31, O, u32>;
#[doc = "Field `AST` reader - Add or subtract time"]
pub type AST_R = crate::BitReader;
#[doc = "Field `AST` writer - Add or subtract time"]
pub type AST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:30 - Timestamp subseconds"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Add or subtract time"]
    #[inline(always)]
    pub fn ast(&self) -> AST_R {
        AST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Timestamp subseconds"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<PTPTSLUD_SPEC, 0> {
        TSS_W::new(self)
    }
    #[doc = "Bit 31 - Add or subtract time"]
    #[inline(always)]
    #[must_use]
    pub fn ast(&mut self) -> AST_W<PTPTSLUD_SPEC, 31> {
        AST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet PTP time stamp low update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptslud::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptslud::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSLUD_SPEC;
impl crate::RegisterSpec for PTPTSLUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptslud::R`](R) reader structure"]
impl crate::Readable for PTPTSLUD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptptslud::W`](W) writer structure"]
impl crate::Writable for PTPTSLUD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTPTSLUD to value 0"]
impl crate::Resettable for PTPTSLUD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
