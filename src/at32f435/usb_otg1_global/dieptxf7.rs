#[doc = "Register `DIEPTXF7` reader"]
pub type R = crate::R<DIEPTXF7_SPEC>;
#[doc = "Register `DIEPTXF7` writer"]
pub type W = crate::W<DIEPTXF7_SPEC>;
#[doc = "Field `INEPTXFSTADDR` reader - IN endpoint FIFO7 transmit RAM start address"]
pub type INEPTXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFSTADDR` writer - IN endpoint FIFO7 transmit RAM start address"]
pub type INEPTXFSTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPTXFDEP` reader - IN endpoint TxFIFO depth"]
pub type INEPTXFDEP_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFDEP` writer - IN endpoint TxFIFO depth"]
pub type INEPTXFDEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFO7 transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxfstaddr(&self) -> INEPTXFSTADDR_R {
        INEPTXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfdep(&self) -> INEPTXFDEP_R {
        INEPTXFDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF7")
            .field("ineptxfstaddr", &self.ineptxfstaddr())
            .field("ineptxfdep", &self.ineptxfdep())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint FIFO7 transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxfstaddr(&mut self) -> INEPTXFSTADDR_W<DIEPTXF7_SPEC> {
        INEPTXFSTADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxfdep(&mut self) -> INEPTXFDEP_W<DIEPTXF7_SPEC> {
        INEPTXFDEP_W::new(self, 16)
    }
}
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF7)\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF7_SPEC;
impl crate::RegisterSpec for DIEPTXF7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf7::R`](R) reader structure"]
impl crate::Readable for DIEPTXF7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptxf7::W`](W) writer structure"]
impl crate::Writable for DIEPTXF7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTXF7 to value 0x0200_0400"]
impl crate::Resettable for DIEPTXF7_SPEC {
    const RESET_VALUE: u32 = 0x0200_0400;
}
