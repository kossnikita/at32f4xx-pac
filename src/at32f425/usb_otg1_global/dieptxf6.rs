#[doc = "Register `DIEPTXF6` reader"]
pub type R = crate::R<DIEPTXF6_SPEC>;
#[doc = "Register `DIEPTXF6` writer"]
pub type W = crate::W<DIEPTXF6_SPEC>;
#[doc = "Field `INEPTXFSTADDR` reader - IN endpoint FIFO6 transmit RAM start address"]
pub type INEPTXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFSTADDR` writer - IN endpoint FIFO6 transmit RAM start address"]
pub type INEPTXFSTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPTXFDEP` reader - IN endpoint TxFIFO depth"]
pub type INEPTXFDEP_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFDEP` writer - IN endpoint TxFIFO depth"]
pub type INEPTXFDEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFO6 transmit RAM start address"]
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
        f.debug_struct("DIEPTXF6")
            .field("ineptxfstaddr", &self.ineptxfstaddr())
            .field("ineptxfdep", &self.ineptxfdep())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint FIFO6 transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxfstaddr(&mut self) -> INEPTXFSTADDR_W<'_, DIEPTXF6_SPEC> {
        INEPTXFSTADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfdep(&mut self) -> INEPTXFDEP_W<'_, DIEPTXF6_SPEC> {
        INEPTXFDEP_W::new(self, 16)
    }
}
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF6)\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF6_SPEC;
impl crate::RegisterSpec for DIEPTXF6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf6::R`](R) reader structure"]
impl crate::Readable for DIEPTXF6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptxf6::W`](W) writer structure"]
impl crate::Writable for DIEPTXF6_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPTXF6 to value 0x0200_0400"]
impl crate::Resettable for DIEPTXF6_SPEC {
    const RESET_VALUE: u32 = 0x0200_0400;
}
