#[doc = "Register `DIEPTXF0` reader"]
pub type R = crate::R<DIEPTXF0_SPEC>;
#[doc = "Register `DIEPTXF0` writer"]
pub type W = crate::W<DIEPTXF0_SPEC>;
#[doc = "Field `INEPT0TXSTADDR` reader - Endpoint 0 transmit RAM start address"]
pub type INEPT0TXSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `INEPT0TXSTADDR` writer - Endpoint 0 transmit RAM start address"]
pub type INEPT0TXSTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPT0TXDEP` reader - Endpoint 0 TxFIFO depth"]
pub type INEPT0TXDEP_R = crate::FieldReader<u16>;
#[doc = "Field `INEPT0TXDEP` writer - Endpoint 0 TxFIFO depth"]
pub type INEPT0TXDEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF0")
            .field("inept0txstaddr", &self.inept0txstaddr())
            .field("inept0txdep", &self.inept0txdep())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn inept0txstaddr(&mut self) -> INEPT0TXSTADDR_W<DIEPTXF0_SPEC> {
        INEPT0TXSTADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn inept0txdep(&mut self) -> INEPT0TXDEP_W<DIEPTXF0_SPEC> {
        INEPT0TXDEP_W::new(self, 16)
    }
}
#[doc = "IN Endpoint TxFIFO 0 transmit FIFO size register (Device mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF0_SPEC;
impl crate::RegisterSpec for DIEPTXF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf0::R`](R) reader structure"]
impl crate::Readable for DIEPTXF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptxf0::W`](W) writer structure"]
impl crate::Writable for DIEPTXF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTXF0 to value 0x0200"]
impl crate::Resettable for DIEPTXF0_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
