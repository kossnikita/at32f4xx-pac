#[doc = "Register `CRC_ADDR` writer"]
pub type W = crate::W<CRC_ADDR_SPEC>;
#[doc = "Field `CRC_ADDR` writer - CRC address"]
pub type CRC_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - CRC address"]
    #[inline(always)]
    #[must_use]
    pub fn crc_addr(&mut self) -> CRC_ADDR_W<CRC_ADDR_SPEC, 0> {
        CRC_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Flash CRC data start address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_addr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_ADDR_SPEC;
impl crate::RegisterSpec for CRC_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crc_addr::W`](W) writer structure"]
impl crate::Writable for CRC_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRC_ADDR to value 0"]
impl crate::Resettable for CRC_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}