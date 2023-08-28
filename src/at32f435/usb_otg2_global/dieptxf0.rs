#[doc = "Register `DIEPTXF0` reader"]
pub type R = crate::R<DIEPTXF0_SPEC>;
#[doc = "Register `DIEPTXF0` writer"]
pub type W = crate::W<DIEPTXF0_SPEC>;
#[doc = "Field `INEPT0TXSTADDR` reader - Endpoint 0 transmit RAM start address"]
pub type INEPT0TXSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `INEPT0TXSTADDR` writer - Endpoint 0 transmit RAM start address"]
pub type INEPT0TXSTADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `INEPT0TXDEP` reader - Endpoint 0 TxFIFO depth"]
pub type INEPT0TXDEP_R = crate::FieldReader<u16>;
#[doc = "Field `INEPT0TXDEP` writer - Endpoint 0 TxFIFO depth"]
pub type INEPT0TXDEP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn inept0txstaddr(&self) -> INEPT0TXSTADDR_R {
        INEPT0TXSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn inept0txdep(&self) -> INEPT0TXDEP_R {
        INEPT0TXDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn inept0txstaddr(&mut self) -> INEPT0TXSTADDR_W<DIEPTXF0_SPEC, 0> {
        INEPT0TXSTADDR_W::new(self)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn inept0txdep(&mut self) -> INEPT0TXDEP_W<DIEPTXF0_SPEC, 16> {
        INEPT0TXDEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IN Endpoint TxFIFO 0 transmit FIFO size register (Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF0_SPEC;
impl crate::RegisterSpec for DIEPTXF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf0::R`](R) reader structure"]
impl crate::Readable for DIEPTXF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptxf0::W`](W) writer structure"]
impl crate::Writable for DIEPTXF0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTXF0 to value 0x0200"]
impl crate::Resettable for DIEPTXF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}